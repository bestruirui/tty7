use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub use crate::core::git::{GitStatus, RepoProbe, RepoSnapshot, probe_repo};
use crate::ui::host_ops::{ByHost, HostId, InFlight};

fn key(host: HostId, path: &Path) -> Cow<'_, Path> {
    tty7_core::core::path_spelling::spelling_on(host, path)
}

#[derive(Default)]
pub struct GitStatusCache {
    roots: ByHost<PathBuf, Option<PathBuf>>,
    homes: ByHost<PathBuf, PathBuf>,
    status: ByHost<PathBuf, GitStatus>,
    probes: InFlight<(HostId, PathBuf)>,
    last_probe: ByHost<PathBuf, Instant>,
}

impl gpui::Global for GitStatusCache {}

impl GitStatusCache {
    pub fn status_for(&self, host: HostId, cwd: &Path) -> Option<GitStatus> {
        let root = self.roots.get(host, &*key(host, cwd))?.as_ref()?;
        self.status.get(host, root.as_path()).cloned()
    }

    pub fn known_repo_for(&self, host: HostId, cwd: &Path) -> Option<Option<PathBuf>> {
        let root = self.roots.get(host, &*key(host, cwd))?;
        Some(root.as_ref().map(|root| {
            self.homes
                .get(host, root)
                .cloned()
                .unwrap_or_else(|| root.clone())
        }))
    }

    pub fn begin_probe(&mut self, host: HostId, cwd: &Path) -> bool {
        let key = (host, key(host, cwd).into_owned());
        if self.probes.begin(key.clone()) {
            true
        } else {
            self.probes.invalidate(&key);
            false
        }
    }

    pub fn begin_probe_throttled(
        &mut self,
        host: HostId,
        cwd: &Path,
        min_interval: Duration,
    ) -> bool {
        if !self.probe_due(host, cwd, min_interval) {
            return false;
        }
        let cwd = key(host, cwd);
        let throttle = self.throttle_key(host, &cwd).to_path_buf();
        self.last_probe.insert(host, throttle, Instant::now());
        self.probes.begin((host, cwd.into_owned()));
        true
    }

    /// Whether [`GitStatusCache::begin_probe_throttled`] would start a probe,
    /// asked without claiming anything.
    ///
    /// For a caller polling on a timer: reaching this cache through
    /// `update_global` wakes everything observing it even when the answer is
    /// "not yet", and a pane asking three times a second redraws the window
    /// three times a second to be told no.
    pub fn probe_due(&self, host: HostId, cwd: &Path, min_interval: Duration) -> bool {
        let cwd = key(host, cwd);
        if self.probes.is_pending(&(host, cwd.to_path_buf())) {
            return false;
        }
        !self
            .last_probe
            .get(host, self.throttle_key(host, &cwd))
            .is_some_and(|at| at.elapsed() < min_interval)
    }

    /// Recheck unresolved and negative answers: repository metadata can be
    /// temporarily unreadable, or a repository can be created in an idle cwd.
    pub fn retry_due(&self, host: HostId, cwd: &Path, min_interval: Duration) -> bool {
        !matches!(self.roots.get(host, &*key(host, cwd)), Some(Some(_)))
            && self.probe_due(host, cwd, min_interval)
    }

    /// `cwd` is already in the cache's own spelling — every caller of this one
    /// has been past [`key`].
    fn throttle_key<'a>(&'a self, host: HostId, cwd: &'a Path) -> &'a Path {
        match self.roots.get(host, cwd) {
            Some(Some(root)) => root,
            _ => cwd,
        }
    }

    pub fn finish_probe(
        &mut self,
        host: HostId,
        cwd: &Path,
        snapshot: Option<RepoSnapshot>,
    ) -> bool {
        // A snapshot arrives spelled by `git`, the cwd by whoever asked for
        // the probe. Both land in the cache's own spelling or the root a
        // status is filed under is not the root the next lookup asks for.
        let cwd = key(host, cwd);
        let snapshot = snapshot.map(|snap| RepoSnapshot {
            root: key(host, &snap.root).into_owned(),
            home: key(host, &snap.home).into_owned(),
            ..snap
        });
        let rerun = !self.probes.finish(&(host, cwd.to_path_buf()));
        let throttle = match &snapshot {
            Some(snap) => snap.root.clone(),
            // A negative result removes the cwd-to-root mapping below, so
            // subsequent retries are throttled by cwd, not the old root.
            None => cwd.to_path_buf(),
        };
        self.last_probe.insert(host, throttle, Instant::now());
        match snapshot {
            Some(snap) => {
                let (added, removed) = snap.counts.unwrap_or_else(|| {
                    self.status
                        .get(host, &snap.root)
                        .map(|g| (g.added, g.removed))
                        .unwrap_or((0, 0))
                });
                self.status.insert(
                    host,
                    snap.root.clone(),
                    GitStatus {
                        branch: snap.branch,
                        added,
                        removed,
                    },
                );
                self.homes.insert(host, snap.root.clone(), snap.home);
                self.roots.insert(host, cwd.to_path_buf(), Some(snap.root));
            }
            None => {
                self.roots.insert(host, cwd.to_path_buf(), None);
            }
        }
        rerun
    }

    /// Land a probe that got no answer ([`RepoProbe::Failed`]).
    ///
    /// Only the bookkeeping moves: the probe is no longer in flight, and the
    /// throttle counts from now so a retry waits its turn. What the cache
    /// knew about `cwd` stays exactly as it was — a repository it had found
    /// is still that repository, and a cwd it had never answered stays
    /// unanswered, which is what lets a tab keep the group it remembers
    /// instead of dropping into Ungrouped on a `git` that did not run.
    pub fn fail_probe(&mut self, host: HostId, cwd: &Path) -> bool {
        let cwd = key(host, cwd);
        let rerun = !self.probes.finish(&(host, cwd.to_path_buf()));
        let throttle = self.throttle_key(host, &cwd).to_path_buf();
        self.last_probe.insert(host, throttle, Instant::now());
        rerun
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const L: HostId = HostId::LOCAL;

    fn snap(root: &str) -> RepoSnapshot {
        RepoSnapshot {
            root: PathBuf::from(root),
            home: PathBuf::from(root),
            branch: "main".into(),
            counts: Some((0, 0)),
        }
    }

    fn wt_snap(root: &str, home: &str) -> RepoSnapshot {
        RepoSnapshot {
            root: PathBuf::from(root),
            home: PathBuf::from(home),
            branch: "main".into(),
            counts: Some((0, 0)),
        }
    }

    #[test]
    fn concurrent_triggers_fold_into_one_probe_then_rerun() {
        let mut cache = GitStatusCache::default();
        let cwd = Path::new("/repo");
        assert!(cache.begin_probe(L, cwd));
        assert!(!cache.begin_probe(L, cwd));
        assert!(cache.finish_probe(L, cwd, Some(snap("/repo"))));
        assert!(cache.begin_probe(L, cwd));
        assert!(!cache.finish_probe(L, cwd, Some(snap("/repo"))));
    }

    #[test]
    fn a_negative_probe_retries_and_recovers_without_a_cwd_change() {
        let mut cache = GitStatusCache::default();
        let cwd = Path::new("/repo/src");
        let gap = Duration::from_secs(10);
        cache.finish_probe(L, cwd, Some(snap("/repo")));
        assert!(!cache.retry_due(L, cwd, Duration::ZERO));

        // Discovery temporarily cannot recognize the repository.
        cache.begin_probe(L, cwd);
        cache.finish_probe(L, cwd, None);
        assert_eq!(cache.known_repo_for(L, cwd), Some(None));
        assert!(
            !cache.retry_due(L, cwd, gap),
            "negative answers are throttled by cwd"
        );
        cache
            .last_probe
            .insert(L, cwd.to_path_buf(), Instant::now() - gap);
        assert!(cache.retry_due(L, cwd, gap));
        assert!(cache.begin_probe_throttled(L, cwd, gap));
        assert!(
            !cache.retry_due(L, cwd, Duration::ZERO),
            "only one retry in flight"
        );

        cache.finish_probe(L, cwd, Some(snap("/repo")));
        assert_eq!(
            cache.known_repo_for(L, cwd),
            Some(Some(PathBuf::from("/repo")))
        );
        assert!(!cache.retry_due(L, cwd, Duration::ZERO));
    }

    #[test]
    fn a_plain_directory_is_rechecked_after_the_retry_interval() {
        let mut cache = GitStatusCache::default();
        let cwd = Path::new("/plain");
        let gap = Duration::from_secs(10);
        assert!(cache.retry_due(L, cwd, gap));
        cache.finish_probe(L, cwd, None);
        assert!(!cache.retry_due(L, cwd, gap));
        cache
            .last_probe
            .insert(L, cwd.to_path_buf(), Instant::now() - gap);
        assert!(cache.retry_due(L, cwd, gap));
    }

    #[test]
    fn known_repo_for_is_three_valued() {
        let mut cache = GitStatusCache::default();
        let (repo, plain, unseen) = (
            Path::new("/repo/a"),
            Path::new("/tmp/x"),
            Path::new("/never"),
        );
        cache.finish_probe(L, repo, Some(snap("/repo")));
        cache.finish_probe(L, plain, None);

        assert_eq!(
            cache.known_repo_for(L, repo),
            Some(Some(PathBuf::from("/repo")))
        );
        assert_eq!(cache.known_repo_for(L, plain), Some(None));
        assert_eq!(cache.known_repo_for(L, unseen), None);
    }

    #[test]
    fn linked_worktrees_group_under_main_repository() {
        let mut cache = GitStatusCache::default();
        let wt = Path::new("/repo/.wt/feat");
        cache.finish_probe(L, wt, Some(wt_snap("/repo/.wt/feat", "/repo")));

        assert_eq!(
            cache.known_repo_for(L, wt),
            Some(Some(PathBuf::from("/repo")))
        );

        let plain = Path::new("/tmp/notes");
        cache.finish_probe(L, plain, None);
        assert_eq!(
            cache.known_repo_for(L, plain),
            Some(None),
            "not a repository"
        );
        assert_eq!(cache.known_repo_for(L, Path::new("/never")), None);
    }

    #[test]
    fn throttled_probes_decline_instead_of_queueing() {
        let mut cache = GitStatusCache::default();
        let cwd = Path::new("/repo");
        let gap = Duration::from_secs(60);

        assert!(cache.begin_probe_throttled(L, cwd, gap));
        assert!(!cache.begin_probe_throttled(L, cwd, gap));
        assert!(!cache.finish_probe(L, cwd, Some(snap("/repo"))));

        assert!(!cache.begin_probe_throttled(L, cwd, gap));
        assert!(cache.begin_probe_throttled(L, cwd, Duration::ZERO));
        assert!(!cache.finish_probe(L, cwd, Some(snap("/repo"))));
        assert!(cache.begin_probe(L, cwd));
    }

    #[test]
    fn throttle_collapses_subdirectories_of_one_repo() {
        let mut cache = GitStatusCache::default();
        let (top, src, docs) = (
            Path::new("/repo"),
            Path::new("/repo/src"),
            Path::new("/repo/docs"),
        );
        let gap = Duration::from_secs(60);

        for cwd in [top, src, docs] {
            assert!(cache.begin_probe_throttled(L, cwd, gap));
            assert!(!cache.finish_probe(L, cwd, Some(snap("/repo"))));
        }

        assert!(!cache.begin_probe_throttled(L, top, gap));
        assert!(!cache.begin_probe_throttled(L, src, gap));

        assert!(cache.begin_probe_throttled(L, docs, Duration::ZERO));
        assert!(!cache.begin_probe_throttled(L, top, gap));
        assert!(!cache.begin_probe_throttled(L, src, gap));

        let other = Path::new("/other");
        assert!(cache.begin_probe_throttled(L, other, gap));
    }
}
