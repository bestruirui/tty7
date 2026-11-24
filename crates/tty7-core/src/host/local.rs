use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use crate::core::git;
use crate::host::{
    Entry, Host, HostId, MTime, Meta, Output, SharedHost, ShellInventory, guard_off_ui,
};

pub struct LocalHost;

impl LocalHost {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> SharedHost {
        Arc::new(LocalHost)
    }

    pub fn shared() -> SharedHost {
        static LOCAL: OnceLock<SharedHost> = OnceLock::new();
        LOCAL.get_or_init(LocalHost::new).clone()
    }

    fn list(&self, dir: &Path) -> io::Result<Vec<(Entry, PathBuf)>> {
        let mut out: Vec<(Entry, PathBuf)> = Vec::new();
        for e in fs::read_dir(dir)?.flatten() {
            let path = e.path();
            let name = e.file_name().to_string_lossy().into_owned();
            let ft = e.file_type().ok();
            let is_symlink = ft.is_some_and(|t| t.is_symlink());
            let is_dir = if is_symlink {
                fs::metadata(&path).map(|m| m.is_dir()).unwrap_or(false)
            } else {
                ft.is_some_and(|t| t.is_dir())
            };
            out.push((
                Entry {
                    name,
                    is_dir,
                    is_symlink,
                },
                path,
            ));
        }

        sort_entries(&mut out);
        Ok(out)
    }
}

fn sort_entries(entries: &mut [(Entry, PathBuf)]) {
    entries.sort_by(|(a, _), (b, _)| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
}

impl Host for LocalHost {
    fn id(&self) -> HostId {
        HostId::LOCAL
    }

    fn separator(&self) -> char {
        std::path::MAIN_SEPARATOR
    }

    fn join(&self, dir: &Path, name: &str) -> PathBuf {
        dir.join(name)
    }

    fn read_dir(&self, dir: &Path) -> io::Result<Vec<Entry>> {
        guard_off_ui();
        Ok(self.list(dir)?.into_iter().map(|(e, _)| e).collect())
    }

    fn stat(&self, p: &Path) -> io::Result<Meta> {
        guard_off_ui();
        let lmd = fs::symlink_metadata(p)?;
        let is_symlink = lmd.file_type().is_symlink();
        let md = if is_symlink { fs::metadata(p)? } else { lmd };
        Ok(Meta {
            is_dir: md.is_dir(),
            is_symlink,
            len: md.len(),
            mtime: md.modified().ok().map(MTime::from_system_time),
            readonly: md.permissions().readonly(),
        })
    }

    fn read_file(&self, p: &Path, max_bytes: u64) -> io::Result<Vec<u8>> {
        guard_off_ui();
        let md = fs::metadata(p)?;
        if md.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::IsADirectory,
                format!("{} is a directory", p.display()),
            ));
        }
        if md.len() > max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::FileTooLarge,
                format!(
                    "{} is {} bytes, over the {max_bytes} limit",
                    p.display(),
                    md.len()
                ),
            ));
        }
        fs::read(p)
    }

    /// The real path behind `p`, in the spelling the rest of tty7 keys by.
    ///
    /// `fs::canonicalize` answers with the extended-length form on Windows —
    /// `\\?\C:\Users\x\repo` — which is a different `Prefix` component from
    /// the `C:\Users\x\repo` a shell, a pane cwd and `git` all report, and so
    /// compares unequal, hashes differently and fails `starts_with` against
    /// every one of them. `\\?\` is a Win32 API escape hatch rather than part
    /// of the path's identity, so it comes off here, at the one boundary that
    /// produces it. What the call is actually *for* — resolving a junction, a
    /// `subst` drive, an 8.3 short name or a symlink — is untouched. See
    /// [`crate::core::path_spelling`].
    fn canonicalize(&self, p: &Path) -> io::Result<PathBuf> {
        guard_off_ui();
        Ok(crate::core::path_spelling::local_spelling_buf(
            fs::canonicalize(p)?,
        ))
    }

    fn write_file(&self, p: &Path, bytes: &[u8]) -> io::Result<Meta> {
        guard_off_ui();
        crate::core::config::write_atomic(p, bytes)?;
        self.stat(p)
    }

    fn create_dir(&self, p: &Path, recursive: bool) -> io::Result<()> {
        guard_off_ui();
        if recursive {
            fs::create_dir_all(p)
        } else {
            fs::create_dir(p)
        }
    }

    fn remove(&self, p: &Path, recursive: bool) -> io::Result<()> {
        guard_off_ui();
        if fs::symlink_metadata(p)?.is_dir() {
            if recursive {
                fs::remove_dir_all(p)
            } else {
                fs::remove_dir(p)
            }
        } else {
            fs::remove_file(p)
        }
    }

    fn git(&self, cwd: &Path, args: &[&str]) -> io::Result<Output> {
        guard_off_ui();
        git::git_output(cwd, args)
    }

    fn repo_head(&self, cwd: &Path) -> Option<Option<git::head::RepoHead>> {
        guard_off_ui();
        Some(git::head::read_head(cwd))
    }

    fn shells(&self) -> io::Result<ShellInventory> {
        guard_off_ui();
        Ok(crate::core::shells::inventory())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_puts_directories_before_files() {
        let mut v: Vec<(Entry, PathBuf)> = ["main.rs", "Cargo.toml", ".gitignore", "src", "Zeta"]
            .iter()
            .map(|n| {
                (
                    Entry {
                        name: (*n).to_string(),
                        is_dir: *n == "src",
                        is_symlink: false,
                    },
                    PathBuf::from(n),
                )
            })
            .collect();
        sort_entries(&mut v);
        let names: Vec<&str> = v.iter().map(|(e, _)| e.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["src", ".gitignore", "Cargo.toml", "main.rs", "Zeta"]
        );
    }

    #[test]
    fn shared_is_a_singleton() {
        assert!(Arc::ptr_eq(&LocalHost::shared(), &LocalHost::shared()));
        assert!(LocalHost::shared().id().is_local());
    }
}
