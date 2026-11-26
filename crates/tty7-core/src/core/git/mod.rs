pub mod head;

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::host::{Host, Output};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GitStatus {
    pub branch: String,
    pub added: u32,
    pub removed: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RepoSnapshot {
    pub root: PathBuf,
    pub home: PathBuf,
    pub branch: String,
    pub counts: Option<(u32, u32)>,
}

/// What asking a directory "which repository are you in" came back with.
///
/// Two ways of not getting a snapshot, and they must not be confused. "Not a
/// repository" is an answer about the directory and holds until it moves; a
/// probe that *failed* — `git` would not start, the Xcode shim behind
/// `/usr/bin/git` had no developer directory to hand off to, a remote link
/// dropped mid-call — says nothing about it at all. Filing the second as the
/// first is what used to pull a tab sitting in a perfectly good repository out
/// of its sidebar group, for good: an idle pane never asks again.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RepoProbe {
    Repo(RepoSnapshot),
    NotARepo,
    Failed,
}

/// Exit status of a `git` that ran and refused: `fatal: not a git repository`,
/// and the handful of other fatals (a `safe.directory` refusal among them)
/// that mean this directory is not one git will work in.
const GIT_FATAL: i32 = 128;

pub fn probe_repo(host: &dyn Host, cwd: &Path) -> RepoProbe {
    let (root, home, branch) = match host.repo_head(cwd) {
        Some(None) => return RepoProbe::NotARepo,
        Some(Some(head)) => (
            crate::core::path_spelling::spelling_on_buf(host.id(), &head.root),
            crate::core::path_spelling::spelling_on_buf(host.id(), &head.home),
            head.branch,
        ),
        None => match locate(host, cwd) {
            Ok((root, home)) => (root, home, None),
            Err(outcome) => return outcome,
        },
    };
    let Some(branch) = branch.or_else(|| {
        if let Some(out) = git(host, cwd, &["symbolic-ref", "--quiet", "--short", "HEAD"]) {
            let name = out.trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
        let sha = git(host, cwd, &["rev-parse", "--short", "HEAD"])?;
        let sha = sha.trim();
        (!sha.is_empty()).then(|| sha.to_string())
    }) else {
        return RepoProbe::Failed;
    };
    let counts = git(host, cwd, &["diff", "--numstat", "HEAD"]).map(|out| {
        let mut added = 0u32;
        let mut removed = 0u32;
        for line in out.lines() {
            let mut fields = line.split('\t');
            if let Some(n) = fields.next().and_then(|s| s.parse::<u32>().ok()) {
                added += n;
            }
            if let Some(n) = fields.next().and_then(|s| s.parse::<u32>().ok()) {
                removed += n;
            }
        }
        (added, removed)
    });
    RepoProbe::Repo(RepoSnapshot {
        root,
        home,
        branch,
        counts,
    })
}

fn git(host: &dyn Host, cwd: &Path, args: &[&str]) -> Option<String> {
    let out = host.git(cwd, args).ok()?;
    if !out.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

/// Root and home of `cwd`'s repository, asked of `git` — for a host that
/// cannot read them in-process ([`Host::repo_head`]).
fn locate(host: &dyn Host, cwd: &Path) -> Result<(PathBuf, PathBuf), RepoProbe> {
    let out = match host.git(
        cwd,
        &[
            "rev-parse",
            "--path-format=absolute",
            "--show-toplevel",
            "--git-dir",
            "--git-common-dir",
        ],
    ) {
        Ok(out) => out,
        // A directory that is gone — a worktree removed under a pane still
        // sitting in it — is not a repository, and never will be again.
        Err(_)
            if host
                .stat(cwd)
                .is_err_and(|e| e.kind() == io::ErrorKind::NotFound) =>
        {
            return Err(RepoProbe::NotARepo);
        }
        Err(_) => return Err(RepoProbe::Failed),
    };
    if !out.success() {
        return Err(match out.status {
            Some(GIT_FATAL) => RepoProbe::NotARepo,
            _ => RepoProbe::Failed,
        });
    }
    let Ok(paths) = String::from_utf8(out.stdout) else {
        return Err(RepoProbe::Failed);
    };
    let mut lines = paths.lines().map(|l| l.trim_end_matches(['\n', '\r']));
    let Some(root) = lines.next().map(|l| git_path(host, l)) else {
        return Err(RepoProbe::Failed);
    };
    let home = repo_home(host, &root, lines.next(), lines.next());
    Ok((root, home))
}

/// A path `git` just printed, in the spelling the rest of tty7 keys by.
///
/// Git for Windows is MSYS2 and answers `rev-parse` with `C:/Users/…` whatever
/// shell asked it. `Path` forgives that much on its own, but the same root also
/// has to compare equal to one that came past `fs::canonicalize` — which spells
/// it `\\?\C:\Users\…`, a different prefix component and so a different key.
/// One spelling at the boundary, rather than a normalisation remembered at each
/// of the places these roots are later compared. See
/// [`crate::core::path_spelling`].
///
/// Asked of `host`, not of `cfg!(windows)`: the same probes run against a
/// remote box, whose `/home/u/src` is native over there and goes straight
/// back over the wire as the cwd of the next `git`. A Windows client
/// re-spelling it would ask a Linux server about `\home\u\src`.
pub(crate) fn git_path(host: &dyn Host, printed: &str) -> PathBuf {
    crate::core::path_spelling::spelling_on_buf(host.id(), printed)
}

pub(crate) fn repo_home(
    host: &dyn Host,
    root: &Path,
    git_dir: Option<&str>,
    common_dir: Option<&str>,
) -> PathBuf {
    let (Some(git_dir), Some(common)) = (git_dir, common_dir) else {
        return root.to_path_buf();
    };
    if git_dir == common {
        return root.to_path_buf();
    }
    let common = git_path(host, common);
    if common.file_name().is_some_and(|name| name == ".git")
        && let Some(parent) = common.parent()
    {
        return parent.to_path_buf();
    }
    common
}

pub fn git_output(cwd: &Path, args: &[&str]) -> io::Result<Output> {
    if !cwd.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("git cwd does not exist: {}", cwd.display()),
        ));
    }
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(cwd)
        .args(args)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let out = crate::core::proc::hide_console(&mut cmd).output()?;
    Ok(Output {
        status: out.status.code(),
        stdout: out.stdout,
        stderr: out.stderr,
    })
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::path::Path;

    /// The `-c` overrides a test's own git invocations carry.
    ///
    /// `user.name`/`user.email` because a CI runner has neither and git refuses
    /// to commit without them, and `commit.gpgsign` because a developer may
    /// have signing on globally. The two line-ending settings are here for the
    /// same reason: Git for Windows ships `core.autocrlf=true` in its *system*
    /// config, so a fixture built with LF and read back is a fixture whose
    /// bytes depend on which machine ran the test. A developer's
    /// `status.showUntrackedFiles=no` would hide the files a test just wrote.
    pub(crate) const PINS: [&str; 12] = [
        "-c",
        "user.name=tty7 test",
        "-c",
        "user.email=test@tty7.invalid",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.autocrlf=false",
        "-c",
        "core.eol=lf",
        "-c",
        "status.showUntrackedFiles=normal",
    ];

    /// The same settings, written into `<repo>/.git/config`.
    ///
    /// Call this straight after `git init`, before anything is written or
    /// checked out, so no blob is ever created under the other rules.
    pub(crate) fn pin_repo_config(repo: &Path) -> bool {
        PINS.chunks(2).all(|pair| {
            let Some((key, value)) = pair[1].split_once('=') else {
                return false;
            };
            super::git_output(repo, &["config", key, value]).is_ok_and(|out| out.success())
        })
    }

    /// A path in the one spelling two halves of an assertion can agree on.
    ///
    /// They do not naturally agree. Anything that came out of `git rev-parse`
    /// is in git's dialect — forward slashes, no extended-length prefix, even
    /// on Windows — while the expected side is usually built from
    /// `fs::canonicalize`, which on Windows answers `\\?\C:\…`. Both name the
    /// same directory and the Win32 file APIs take either, so the code under
    /// test is right to pass git's answer straight through; it is only the
    /// comparison that has to pick a spelling. On unix this is the identity.
    ///
    /// Note this normalises the *spelling*, not the path: equality stays exact.
    pub(crate) fn one_spelling(p: &Path) -> String {
        let slashed = p.to_string_lossy().replace('\\', "/");
        match slashed.strip_prefix("//?/") {
            Some(bare) => bare.to_string(),
            None => slashed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::one_spelling;
    use super::*;

    fn h() -> crate::host::SharedHost {
        crate::host::local::LocalHost::new()
    }

    /// The mapping [`test_support::one_spelling`] promises, proven on literals
    /// rather than on whatever this machine's temp directory happens to be —
    /// a developer on unix never sees either of the Windows shapes, which is
    /// exactly how a comparison against `fs::canonicalize` reached Windows CI
    /// unnoticed in the first place.
    #[test]
    fn one_spelling_folds_the_two_ways_windows_writes_a_path() {
        assert_eq!(
            one_spelling(Path::new(r"\\?\C:\Users\x\repo\.git")),
            "C:/Users/x/repo/.git",
            "the extended-length prefix goes, and the separators match git's"
        );
        assert_eq!(
            one_spelling(Path::new(r"C:\Users\x\repo\.git")),
            "C:/Users/x/repo/.git",
            "a plain Windows path lands on the same spelling"
        );
        assert_eq!(
            one_spelling(Path::new("C:/Users/x/repo/.git")),
            "C:/Users/x/repo/.git",
            "what git itself answers is already in that spelling"
        );
        assert_eq!(
            one_spelling(Path::new("/private/var/t/repo/.git")),
            "/private/var/t/repo/.git",
            "on unix it is the identity, which is why this never fired locally"
        );
    }

    #[test]
    fn a_plain_or_vanished_directory_is_not_a_repo() {
        let host = h();
        let dir = tempfile::tempdir().unwrap();
        let plain = dir.path().join("plain");
        std::fs::create_dir(&plain).unwrap();
        // A temp directory can sit under a repository of its own on some
        // machine; git has nothing to say about this test's premise there.
        if host
            .git(&plain, &["rev-parse", "--git-dir"])
            .is_ok_and(|o| o.success())
        {
            return;
        }
        assert_eq!(probe_repo(&*host, &plain), RepoProbe::NotARepo);
        assert_eq!(
            probe_repo(&*host, &dir.path().join("gone")),
            RepoProbe::NotARepo,
            "a removed worktree is not a failed probe"
        );
    }

    #[test]
    fn own_repo_has_a_root() {
        let here = env!("CARGO_MANIFEST_DIR");
        if let RepoProbe::Repo(snap) = probe_repo(&*h(), Path::new(here)) {
            assert!(Path::new(here).starts_with(&snap.root));
        }
    }
    #[test]
    fn repo_home_resolves_worktree_layouts() {
        let host = h();
        let host = &*host;
        let root = Path::new("/repo/.wt/feat");

        assert_eq!(
            repo_home(
                host,
                Path::new("/repo"),
                Some("/repo/.git"),
                Some("/repo/.git")
            ),
            PathBuf::from("/repo")
        );
        assert_eq!(
            repo_home(
                host,
                root,
                Some("/repo/.git/worktrees/feat"),
                Some("/repo/.git")
            ),
            PathBuf::from("/repo")
        );
        assert_eq!(
            repo_home(
                host,
                root,
                Some("/bare.git/worktrees/feat"),
                Some("/bare.git")
            ),
            PathBuf::from("/bare.git")
        );
        assert_eq!(
            repo_home(host, root, Some("/repo/.git"), None),
            root.to_path_buf()
        );
        assert_eq!(repo_home(host, root, None, None), root.to_path_buf());
    }

    /// The root every git probe answers with is the directory the *OS* names,
    /// compared as a plain `PathBuf` — which is how every consumer compares
    /// it.
    ///
    /// Not gated to any platform, deliberately. Git for Windows is MSYS2 and
    /// prints `C:/Users/…`; `Host::canonicalize` used to answer `\\?\C:\Users\
    /// …`; those are different `Prefix` components, so the two never matched
    /// and repository caches keyed by one missed the other. Nothing compared
    /// them on Windows, which is exactly why nobody noticed — a `#[cfg(unix)]`
    /// on this would put it straight back.
    #[test]
    fn a_probed_root_is_the_same_path_the_host_canonicalizes_to() {
        let host = h();
        let dir = std::env::temp_dir().join(format!("tty7-root-spelling-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let made = host
            .git(&dir, &["init", "--quiet"])
            .is_ok_and(|out| out.success());
        if !made {
            let _ = std::fs::remove_dir_all(&dir);
            return; // no git on this machine
        }
        assert!(super::test_support::pin_repo_config(&dir));
        std::fs::write(dir.join("a.txt"), "one\n").unwrap();
        let mut commit = super::test_support::PINS.to_vec();
        commit.extend_from_slice(&["commit", "--quiet", "-m", "base"]);
        assert!(
            host.git(&dir, &["add", "-A"])
                .is_ok_and(|out| out.success())
        );
        assert!(host.git(&dir, &commit).is_ok_and(|out| out.success()));

        // The one directory, under the two names this process can learn it by:
        // what the OS handed back, and what resolving it answers.
        let canonical = host.canonicalize(&dir).expect("the scratch dir resolves");

        let RepoProbe::Repo(snap) = probe_repo(&*host, &dir) else {
            panic!("a repository was just created here");
        };
        assert_eq!(
            snap.root, canonical,
            "the probed root and the resolved directory are one key"
        );
        assert_eq!(snap.home, canonical, "and so is a non-worktree's home");

        // Asking from the resolved spelling has to reach the same answer, or
        // a pane whose cwd arrived that way lands in a second repository.
        let RepoProbe::Repo(from_canonical) = probe_repo(&*host, &canonical) else {
            panic!("the same repository, asked from its other name");
        };
        assert_eq!(from_canonical.root, snap.root);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
