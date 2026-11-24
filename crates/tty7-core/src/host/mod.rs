pub mod conformance;
pub mod local;
pub mod remote;
pub mod server;

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::OnceLock;
use std::thread::ThreadId;

pub use crate::core::shells::ShellInventory;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct HostId(pub u64);

impl HostId {
    pub const LOCAL: HostId = HostId(0);

    pub fn from_connection_key(key: &str) -> HostId {
        let h = fnv1a64(key.as_bytes());
        HostId(if h == 0 { 1 } else { h })
    }

    pub fn is_local(self) -> bool {
        self == HostId::LOCAL
    }
}

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

static UI_THREAD: OnceLock<ThreadId> = OnceLock::new();

pub fn register_ui_thread() {
    let _ = UI_THREAD.set(std::thread::current().id());
}

pub fn is_ui_thread() -> bool {
    UI_THREAD
        .get()
        .is_some_and(|t| *t == std::thread::current().id())
}

#[inline]
pub fn guard_off_ui() {
    debug_assert!(
        !is_ui_thread(),
        "Host call on the UI thread — route it through ui::host_ops::HostOps"
    );
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub name: String,
    pub is_dir: bool,
    pub is_symlink: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Meta {
    pub is_dir: bool,
    pub is_symlink: bool,
    pub len: u64,
    pub mtime: Option<MTime>,
    pub readonly: bool,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct MTime {
    pub secs: i64,
    pub nanos: u32,
}

impl MTime {
    pub fn from_system_time(t: std::time::SystemTime) -> MTime {
        match t.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => MTime {
                secs: d.as_secs() as i64,
                nanos: d.subsec_nanos(),
            },
            Err(e) => {
                let d = e.duration();
                let (secs, nanos) = if d.subsec_nanos() == 0 {
                    (-(d.as_secs() as i64), 0)
                } else {
                    (-(d.as_secs() as i64) - 1, 1_000_000_000 - d.subsec_nanos())
                };
                MTime { secs, nanos }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub status: Option<i32>,
    #[serde(with = "b64")]
    pub stdout: Vec<u8>,
    #[serde(with = "b64")]
    pub stderr: Vec<u8>,
}

pub(crate) mod b64 {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    use serde::{Deserialize as _, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(d)?;
        STANDARD.decode(&s).map_err(serde::de::Error::custom)
    }
}

impl Output {
    pub fn success(&self) -> bool {
        self.status == Some(0)
    }
}

pub trait Host: Send + Sync + 'static {
    fn id(&self) -> HostId;

    /// How far away this host is: the round trip to it, measured now.
    ///
    /// `None` from a host with no link to measure — the local one, whose
    /// "peer" is this process's own daemon over a Unix socket — and from a
    /// remote one whose link is down or has not answered a ping yet.
    fn link_rtt(&self) -> Option<std::time::Duration> {
        None
    }

    fn separator(&self) -> char;

    fn join(&self, dir: &Path, name: &str) -> PathBuf {
        default_join(dir, name, self.separator())
    }

    fn read_dir(&self, dir: &Path) -> io::Result<Vec<Entry>>;

    fn stat(&self, p: &Path) -> io::Result<Meta>;

    fn exists(&self, p: &Path) -> bool {
        self.stat(p).is_ok()
    }

    fn read_file(&self, p: &Path, max_bytes: u64) -> io::Result<Vec<u8>>;

    fn canonicalize(&self, p: &Path) -> io::Result<PathBuf>;

    fn write_file(&self, p: &Path, bytes: &[u8]) -> io::Result<Meta>;

    fn create_dir(&self, p: &Path, recursive: bool) -> io::Result<()>;

    fn remove(&self, p: &Path, recursive: bool) -> io::Result<()>;

    fn git(&self, cwd: &Path, args: &[&str]) -> io::Result<Output>;

    /// The repository `cwd` is in, answered without running `git` — or
    /// `None` when this host cannot, and [`crate::core::git::probe_repo`]
    /// asks `git` instead.
    ///
    /// Only this machine answers. A remote host could read the same files on
    /// its side, but asking it to would be a request its server has never
    /// heard of, and one of those takes the whole link down; the `git` it
    /// runs over there has no Xcode shim to trip on anyway.
    fn repo_head(&self, _cwd: &Path) -> Option<Option<crate::core::git::head::RepoHead>> {
        None
    }

    fn shells(&self) -> io::Result<ShellInventory>;

    fn is_connected(&self) -> bool {
        true
    }

    /// What is running inside one of this host's panes, and what it is
    /// listening on — or `None` where this host cannot say.
    ///
    /// `None` is not "nothing is running": it is the answer from a host whose
    /// panes are somebody else's to describe. The local host gives it, because
    /// its panes belong to the daemon the caller asks directly; so does a peer
    /// too old to know the request. Callers must keep the two apart — an empty
    /// list means the pane really is serving nothing, and drawing "no ports"
    /// over "we could not ask" is how a remote pane came to look idle while a
    /// dev server was up in it.
    fn pane_procs(&self, _pane_id: u64) -> Option<crate::daemon::protocol::PaneProcs> {
        None
    }
}

pub fn is_content_change(kind: &notify::EventKind) -> bool {
    use notify::event::{AccessKind, AccessMode};

    !matches!(
        kind,
        notify::EventKind::Access(
            AccessKind::Open(_) | AccessKind::Read | AccessKind::Close(AccessMode::Read),
        )
    )
}

pub fn default_join(dir: &Path, name: &str, sep: char) -> PathBuf {
    let mut s = dir.to_string_lossy().into_owned();
    if !s.is_empty() && !s.ends_with(sep) && !s.ends_with('/') {
        s.push(sep);
    }
    s.push_str(name);
    PathBuf::from(s)
}

pub type SharedHost = Arc<dyn Host>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a64_matches_the_published_vectors() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
    }

    #[test]
    fn zero_is_reserved_for_local() {
        assert!(HostId::LOCAL.is_local());
        for i in 0..2000u32 {
            let key = format!("ssh-direct:me@box{i}:22");
            assert!(!HostId::from_connection_key(&key).is_local());
        }
        assert!(!HostId::from_connection_key("").is_local());
    }

    #[test]
    fn connection_keys_map_to_stable_ids() {
        let a = HostId::from_connection_key("ssh-direct:me@box:22");
        assert_eq!(a, HostId::from_connection_key("ssh-direct:me@box:22"));
        assert_ne!(a, HostId::from_connection_key("ssh-direct:me@box:2222"));
        assert_ne!(a, HostId::from_connection_key("wsl:Ubuntu"));
    }

    #[test]
    fn default_join_uses_the_given_separator() {
        assert_eq!(
            default_join(Path::new("/home/me"), "src", '/'),
            PathBuf::from("/home/me/src")
        );
        assert_eq!(
            default_join(Path::new("/"), "etc", '/'),
            PathBuf::from("/etc")
        );
        assert_eq!(
            default_join(Path::new("/home/me/"), "src", '/'),
            PathBuf::from("/home/me/src")
        );
        assert_eq!(
            default_join(Path::new(r"C:\Users"), "me", '\\'),
            PathBuf::from(r"C:\Users\me")
        );
    }

    #[test]
    fn mtime_handles_both_sides_of_the_epoch() {
        use std::time::{Duration, UNIX_EPOCH};
        let t = UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_700);
        assert_eq!(
            MTime::from_system_time(t),
            MTime {
                secs: 1_700_000_000,
                nanos: 123_456_700
            }
        );
        assert_eq!(
            MTime::from_system_time(UNIX_EPOCH),
            MTime { secs: 0, nanos: 0 }
        );
        let before = UNIX_EPOCH - Duration::new(1, 500_000_000);
        assert_eq!(
            MTime::from_system_time(before),
            MTime {
                secs: -2,
                nanos: 500_000_000
            }
        );
    }

    #[test]
    fn output_success_is_exit_zero_only() {
        let ok = Output {
            status: Some(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };
        assert!(ok.success());
        let bad = Output {
            status: Some(128),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };
        assert!(!bad.success());
        let signalled = Output {
            status: None,
            stdout: Vec::new(),
            stderr: Vec::new(),
        };
        assert!(!signalled.success());
    }

    #[test]
    fn output_bytes_cross_json_as_base64() {
        let o = Output {
            status: Some(1),
            stdout: vec![0x00, 0xff, 0x80, b'h', b'i'],
            stderr: b"boom".to_vec(),
        };
        let json = serde_json::to_string(&o).unwrap();
        assert!(json.contains("\"stdout\":\"AP+AaGk=\""), "{json}");
        assert!(
            !json.contains('['),
            "bytes must not render as an array: {json}"
        );
        assert_eq!(serde_json::from_str::<Output>(&json).unwrap(), o);
    }

    #[test]
    fn the_ui_guard_is_inert_without_registration() {
        guard_off_ui();
    }

    #[test]
    fn host_is_object_safe() {
        fn takes_dyn(_h: &dyn Host) {}
        let h: SharedHost = local::LocalHost::new();
        takes_dyn(&*h);
    }
}
