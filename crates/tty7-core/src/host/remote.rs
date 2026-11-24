use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use crate::daemon::control::{
    ControlClient, ControlHello, ControlHelloOk, ControlRequest, EventSink, KEEPALIVE_DEAD_AFTER,
    KEEPALIVE_IDLE_BEFORE_PING, KEEPALIVE_PING_INTERVAL, LinkShutdown, ReplyOk,
};
use crate::host::{Entry, Host, HostId, Meta, Output, SharedHost, ShellInventory};

pub struct RemoteHost {
    id: HostId,
    client: Arc<ControlClient>,
    separator: char,
}

impl RemoteHost {
    pub fn connect<R, W>(
        r: R,
        w: W,
        connection_key: &str,
        hello: &ControlHello,
    ) -> io::Result<Arc<RemoteHost>>
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        Self::connect_with(r, w, None, connection_key, hello)
    }

    pub fn over_tcp(
        sock: std::net::TcpStream,
        connection_key: &str,
        hello: &ControlHello,
    ) -> io::Result<Arc<RemoteHost>> {
        let r = sock.try_clone()?;
        let closer: Arc<dyn LinkShutdown> = Arc::new(sock.try_clone()?);
        Self::connect_with(r, sock, Some(closer), connection_key, hello)
    }

    #[cfg(unix)]
    pub fn over_unix(
        sock: std::os::unix::net::UnixStream,
        connection_key: &str,
        hello: &ControlHello,
    ) -> io::Result<Arc<RemoteHost>> {
        let r = sock.try_clone()?;
        let closer: Arc<dyn LinkShutdown> = Arc::new(sock.try_clone()?);
        Self::connect_with(r, sock, Some(closer), connection_key, hello)
    }

    pub fn connect_with<R, W>(
        r: R,
        w: W,
        shutdown: Option<Arc<dyn LinkShutdown>>,
        connection_key: &str,
        hello: &ControlHello,
    ) -> io::Result<Arc<RemoteHost>>
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let id = HostId::from_connection_key(connection_key);
        let sink: EventSink =
            Box::new(move |event| crate::daemon::control::observe_event(id, event));

        let client = Arc::new(ControlClient::connect_with(r, w, shutdown, hello, sink)?);
        let separator = client.hello().separator;
        let host = Arc::new(RemoteHost {
            id,
            client: Arc::clone(&client),
            separator,
        });
        spawn_keepalive(Arc::downgrade(&client));
        Ok(host)
    }

    pub fn peer(&self) -> &ControlHelloOk {
        self.client.hello()
    }

    pub fn home(&self) -> PathBuf {
        PathBuf::from(&self.client.hello().home)
    }

    pub fn client(&self) -> &Arc<ControlClient> {
        &self.client
    }

    pub fn into_shared(self: Arc<Self>) -> SharedHost {
        self
    }

    fn call(&self, req: ControlRequest) -> io::Result<ReplyOk> {
        self.client.call(req)
    }
}

fn wire_path(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn wrong_shape(expected: &str, got: &ReplyOk) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("control peer answered with {got:?} where {expected} was required"),
    )
}

impl Host for RemoteHost {
    fn id(&self) -> HostId {
        self.id
    }

    fn separator(&self) -> char {
        self.separator
    }

    fn read_dir(&self, dir: &Path) -> io::Result<Vec<Entry>> {
        match self.call(ControlRequest::ReadDir {
            dir: wire_path(dir),
        })? {
            ReplyOk::Entries(e) => Ok(e),
            other => Err(wrong_shape("a directory listing", &other)),
        }
    }

    fn stat(&self, p: &Path) -> io::Result<Meta> {
        match self.call(ControlRequest::Stat { path: wire_path(p) })? {
            ReplyOk::Meta(m) => Ok(m),
            other => Err(wrong_shape("file metadata", &other)),
        }
    }

    fn exists(&self, p: &Path) -> bool {
        matches!(
            self.call(ControlRequest::Exists { path: wire_path(p) }),
            Ok(ReplyOk::Bool(true))
        )
    }

    fn read_file(&self, p: &Path, max_bytes: u64) -> io::Result<Vec<u8>> {
        let got = self.client.call_full(
            ControlRequest::ReadFile {
                path: wire_path(p),
                max_bytes,
            },
            &[],
        )?;
        match got.reply {
            ReplyOk::FileMeta { .. } => Ok(got.blob),
            other => Err(wrong_shape("file contents", &other)),
        }
    }

    fn canonicalize(&self, p: &Path) -> io::Result<PathBuf> {
        match self.call(ControlRequest::Canonicalize { path: wire_path(p) })? {
            ReplyOk::Path(s) => Ok(PathBuf::from(s)),
            other => Err(wrong_shape("a canonical path", &other)),
        }
    }

    fn write_file(&self, p: &Path, bytes: &[u8]) -> io::Result<Meta> {
        match self
            .client
            .call_with_blob(ControlRequest::WriteFile { path: wire_path(p) }, bytes)?
        {
            ReplyOk::Meta(meta) => Ok(meta),
            other => Err(wrong_shape("file metadata", &other)),
        }
    }

    fn create_dir(&self, p: &Path, recursive: bool) -> io::Result<()> {
        self.expect_unit(ControlRequest::CreateDir {
            path: wire_path(p),
            recursive,
        })
    }

    fn remove(&self, p: &Path, recursive: bool) -> io::Result<()> {
        self.expect_unit(ControlRequest::Remove {
            path: wire_path(p),
            recursive,
        })
    }

    fn link_rtt(&self) -> Option<std::time::Duration> {
        // A ping of our own rather than whatever the keepalive last left
        // behind: that one only fires on an idle link, and a link being polled
        // for this is by definition not idle, so its number would age out of
        // date exactly while someone is watching it.
        if self.client.is_connected()
            && let Err(e) = self.client.ping()
        {
            // The last good measurement below still stands. A link that has
            // just gone down reports the distance it had while it was up,
            // which is better than a blank until something notices it is gone.
            log::debug!("could not ping {:?}: {e}", self.id);
        }
        self.client.last_rtt()
    }

    /// The peer owns these panes' PTYs, so it is the one that can walk their
    /// process trees. A peer that does not announce the feature is not asked:
    /// it would answer `Err` and the caller cannot tell that apart from a pane
    /// serving nothing.
    fn pane_procs(&self, pane_id: u64) -> Option<crate::daemon::protocol::PaneProcs> {
        if !self
            .peer()
            .has_feature(crate::daemon::control::feature::PANE_PROCS)
        {
            return None;
        }
        match self.call(ControlRequest::PaneProcs { pane_id }) {
            Ok(ReplyOk::PaneProcs(procs)) => Some(procs),
            Ok(other) => {
                log::warn!("PaneProcs answered with {other:?}");
                None
            }
            Err(e) => {
                log::debug!("could not read pane {pane_id}'s processes: {e}");
                None
            }
        }
    }

    fn git(&self, cwd: &Path, args: &[&str]) -> io::Result<Output> {
        match self.call(ControlRequest::Git {
            cwd: wire_path(cwd),
            args: args.iter().map(|a| a.to_string()).collect(),
        })? {
            ReplyOk::Output(o) => Ok(o),
            other => Err(wrong_shape("a process result", &other)),
        }
    }

    fn shells(&self) -> io::Result<ShellInventory> {
        match self.call(ControlRequest::Shells)? {
            ReplyOk::Shells(inv) => Ok(inv),
            other => Err(wrong_shape("a shell inventory", &other)),
        }
    }

    fn is_connected(&self) -> bool {
        self.client.is_connected()
    }
}

impl RemoteHost {
    fn expect_unit(&self, req: ControlRequest) -> io::Result<()> {
        match self.call(req)? {
            ReplyOk::Unit => Ok(()),
            other => Err(wrong_shape("an acknowledgement", &other)),
        }
    }
}

impl std::fmt::Debug for RemoteHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteHost")
            .field("id", &self.id)
            .field("separator", &self.separator)
            .field("connected", &self.is_connected())
            .finish()
    }
}

fn spawn_keepalive(client: Weak<ControlClient>) {
    let _ = std::thread::Builder::new()
        .name("tty7-control-keepalive".into())
        .spawn(move || {
            let mut last_ping = Instant::now();
            loop {
                std::thread::sleep(Duration::from_secs(1));
                let Some(client) = client.upgrade() else {
                    return;
                };
                if !client.is_connected() {
                    return;
                }
                let idle = client.idle_for();
                if idle >= KEEPALIVE_DEAD_AFTER {
                    log::warn!("control link silent for {idle:?}; treating it as dead",);
                    client.close();
                    return;
                }
                if idle >= KEEPALIVE_IDLE_BEFORE_PING
                    && last_ping.elapsed() >= KEEPALIVE_PING_INTERVAL
                {
                    last_ping = Instant::now();
                    if let Err(e) = client.ping() {
                        log::debug!("control keepalive ping failed: {e}");
                    }
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::control::{
        ControlClientMsg, ControlReply, ControlServerMsg, MTime, WireError, WireErrorKind,
    };
    use std::net::{TcpListener, TcpStream};
    use std::sync::atomic::Ordering;
    use std::sync::mpsc;
    use std::thread;

    fn hello_ok(separator: char) -> ControlHelloOk {
        ControlHelloOk {
            control_version: crate::daemon::control::CONTROL_VERSION,
            protocol_version: crate::daemon::protocol::PROTOCOL_VERSION,
            build: "test".into(),
            separator,
            home: "/home/me".into(),
            features: vec![
                crate::daemon::control::feature::CONTROL.into(),
                crate::daemon::control::feature::HOST_RPC.into(),
            ],
            instance: "test-instance".into(),
        }
    }

    fn meta() -> Meta {
        Meta {
            is_dir: false,
            is_symlink: false,
            len: 12,
            mtime: Some(MTime {
                secs: 1_769_000_000,
                nanos: 7,
            }),
            readonly: false,
        }
    }

    fn host_with_peer<F>(
        separator: char,
        answer: F,
    ) -> (Arc<RemoteHost>, mpsc::Receiver<ControlRequest>)
    where
        F: Fn(&ControlRequest) -> Option<(ControlReply, Vec<u8>)> + Send + 'static,
    {
        host_with_peer_announcing(separator, &[], answer)
    }

    /// [`host_with_peer`], with a peer whose hello names `extra` features on
    /// top of the two every control peer has.
    fn host_with_peer_announcing<F>(
        separator: char,
        extra: &[&str],
        answer: F,
    ) -> (Arc<RemoteHost>, mpsc::Receiver<ControlRequest>)
    where
        F: Fn(&ControlRequest) -> Option<(ControlReply, Vec<u8>)> + Send + 'static,
    {
        let mut hello = hello_ok(separator);
        hello.features.extend(extra.iter().map(|f| f.to_string()));
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (seen_tx, seen_rx) = mpsc::channel();
        thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            match ControlClientMsg::read(&mut sock).unwrap() {
                ControlClientMsg::Hello(_) => {}
                other => panic!("expected Hello, got {other:?}"),
            }
            ControlServerMsg::HelloOk(hello).encode(&mut sock).unwrap();
            sock.flush().unwrap();
            loop {
                let (req_id, req) = match ControlClientMsg::read(&mut sock) {
                    Ok(ControlClientMsg::Request { req_id, req }) => (req_id, req),
                    Ok(ControlClientMsg::RequestBlob { req_id, req, blob }) => {
                        let _ = blob;
                        (req_id, req)
                    }
                    _ => return,
                };
                let reply = answer(&req);
                seen_tx.send(req).unwrap();
                match reply {
                    Some((reply, blob)) if blob.is_empty() => {
                        ControlServerMsg::Response { req_id, reply }
                            .encode(&mut sock)
                            .unwrap();
                    }
                    Some((reply, blob)) => {
                        ControlServerMsg::ResponseBlob {
                            req_id,
                            reply,
                            blob,
                        }
                        .encode(&mut sock)
                        .unwrap();
                    }
                    None => return,
                }
                sock.flush().unwrap();
            }
        });

        let sock = TcpStream::connect(addr).unwrap();
        let host = RemoteHost::over_tcp(
            sock,
            "ssh-alias:testbox",
            &ControlHello::host_rpc("tok", "laptop"),
        )
        .unwrap();
        (host, seen_rx)
    }

    #[test]
    fn link_rtt_pings_and_reports_what_came_back() {
        let (host, seen) = host_with_peer('/', |req| match req {
            ControlRequest::Ping => Some((ControlReply::Ok(ReplyOk::Pong), vec![])),
            other => panic!("unexpected request {other:?}"),
        });

        assert!(
            host.link_rtt().is_some(),
            "a ping that came back is a measurement"
        );
        assert_eq!(seen.recv().unwrap(), ControlRequest::Ping);
        assert_eq!(
            seen.try_recv().ok(),
            None,
            "one row is worth one round trip and no more"
        );
    }

    /// The latency row must mean the distance to the peer, not how long the
    /// peer spent on whatever was asked of it. Timing every call would put a
    /// `ReadFile` of a large file, or a `Git` that shells out, on that row and
    /// read as a network gone seconds slow.
    #[test]
    fn only_a_ping_is_timed() {
        let (host, _seen) = host_with_peer('/', |req| match req {
            ControlRequest::Ping => Some((ControlReply::Ok(ReplyOk::Pong), vec![])),
            ControlRequest::Exists { .. } => Some((ControlReply::Ok(ReplyOk::Bool(true)), vec![])),
            other => panic!("unexpected request {other:?}"),
        });

        assert_eq!(
            host.client().last_rtt(),
            None,
            "a link nothing has pinged has no measurement to report"
        );
        assert!(host.exists(Path::new("/etc/hosts")));
        assert_eq!(
            host.client().last_rtt(),
            None,
            "an ordinary call measures the peer's work, so it leaves the link's latency alone"
        );
        host.client().ping().unwrap();
        assert!(host.client().last_rtt().is_some());
    }

    /// A link that has just gone down keeps the distance it had while it was
    /// up. Blanking the row on the first failed ping would take the number
    /// away at exactly the moment someone is looking at it to work out why the
    /// pane has stopped responding.
    #[test]
    fn a_link_that_goes_down_keeps_its_last_measurement() {
        let served = std::sync::atomic::AtomicBool::new(true);
        let (host, _seen) = host_with_peer('/', move |req| match req {
            ControlRequest::Ping => match served.swap(false, Ordering::Relaxed) {
                true => Some((ControlReply::Ok(ReplyOk::Pong), vec![])),
                // Hanging up rather than answering, which is what a peer whose
                // machine went away looks like from here.
                false => None,
            },
            other => panic!("unexpected request {other:?}"),
        });

        let measured = host.link_rtt().expect("the first ping came back");
        assert_eq!(host.link_rtt(), Some(measured));
        assert!(!host.is_connected(), "the second ping took the link down");
        assert_eq!(host.link_rtt(), Some(measured));
    }

    #[test]
    fn shells_come_from_the_peer() {
        let (host, seen) = host_with_peer('/', |req| match req {
            ControlRequest::Shells => Some((
                ControlReply::Ok(ReplyOk::Shells(crate::core::shells::ShellInventory {
                    shells: vec![crate::core::shells::DetectedShell {
                        label: "zsh".into(),
                        program: "/usr/bin/zsh".into(),
                        args: vec!["--no-rcs".into()],
                        args_are_tty7_defaults: false,
                        user_authored: false,
                    }],
                    default_name: "zsh".into(),
                })),
                vec![],
            )),
            other => panic!("unexpected request {other:?}"),
        });

        let inv = host.shells().unwrap();
        assert_eq!(seen.recv().unwrap(), ControlRequest::Shells);
        assert_eq!(inv.default_name, "zsh");
        assert_eq!(inv.shells[0].program, "/usr/bin/zsh");
        assert_eq!(inv.shells[0].args, ["--no-rcs"]);
        assert!(!inv.shells[0].args_are_tty7_defaults);
    }

    /// A remote host has no in-process answer, so the repo probe asks the
    /// far side's `git` — and has to tell a refusal about the directory from
    /// a `git` that never ran, or a tab loses its group to a flaky link.
    #[test]
    fn a_remote_repo_probe_tells_not_a_repo_from_a_failure() {
        use crate::core::git::{RepoProbe, probe_repo};
        let out = |status: i32, stdout: &str| {
            ControlReply::Ok(ReplyOk::Output(Output {
                status: Some(status),
                stdout: stdout.as_bytes().to_vec(),
                stderr: Vec::new(),
            }))
        };
        let (host, _seen) = host_with_peer('/', move |req| {
            let reply = match req {
                ControlRequest::Git { cwd, args } => match (cwd.as_str(), args[0].as_str()) {
                    ("/repo", "rev-parse") => out(0, "/repo\n/repo/.git\n/repo/.git\n"),
                    ("/plain", _) => out(128, ""),
                    ("/shim", _) => out(1, ""),
                    ("/gone", _) => {
                        ControlReply::Err(WireError::new(WireErrorKind::NotFound, "no cwd"))
                    }
                    _ => ControlReply::Err(WireError::new(WireErrorKind::ConnectionReset, "")),
                },
                ControlRequest::Stat { path } if path == "/gone" => {
                    ControlReply::Err(WireError::new(WireErrorKind::NotFound, "gone"))
                }
                ControlRequest::Stat { .. } => ControlReply::Ok(ReplyOk::Meta(meta())),
                other => panic!("unexpected request {other:?}"),
            };
            Some((reply, vec![]))
        });
        let h: &dyn Host = host.as_ref();

        let RepoProbe::Repo(snap) = probe_repo(h, Path::new("/repo")) else {
            panic!("/repo is a repository");
        };
        assert_eq!(snap.root, Path::new("/repo"));
        assert_eq!(snap.home, Path::new("/repo"));
        assert_eq!(probe_repo(h, Path::new("/plain")), RepoProbe::NotARepo);
        assert_eq!(probe_repo(h, Path::new("/gone")), RepoProbe::NotARepo);
        assert_eq!(probe_repo(h, Path::new("/shim")), RepoProbe::Failed);
        assert_eq!(probe_repo(h, Path::new("/flaky")), RepoProbe::Failed);
    }

    #[test]
    fn path_arithmetic_follows_the_peer_not_the_client() {
        let (host, _seen) =
            host_with_peer('/', |_| Some((ControlReply::Ok(ReplyOk::Unit), vec![])));
        let host: &dyn Host = host.as_ref();

        assert_eq!(host.separator(), '/');
        assert_eq!(
            host.join(Path::new("/home/me"), "src"),
            PathBuf::from("/home/me/src"),
            "joins with the remote's separator on every client platform"
        );
    }

    #[test]
    fn a_windows_peer_gets_windows_path_semantics() {
        let (host, _seen) =
            host_with_peer('\\', |_| Some((ControlReply::Ok(ReplyOk::Unit), vec![])));
        let host: &dyn Host = host.as_ref();

        assert_eq!(host.separator(), '\\');
        assert_eq!(
            host.join(Path::new("C:\\src"), "main.rs"),
            PathBuf::from("C:\\src\\main.rs")
        );
    }

    #[test]
    fn read_file_over_the_limit_fails_without_transferring() {
        let (host, _seen) = host_with_peer('/', |_| {
            Some((
                ControlReply::Err(WireError::new(
                    WireErrorKind::FileTooLarge,
                    "/big is 900 MB, over the 10 MB limit",
                )),
                vec![],
            ))
        });
        let e = host
            .read_file(Path::new("/big"), 10 * 1024 * 1024)
            .unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::FileTooLarge);
        assert!(e.to_string().contains("900 MB"));
    }

    #[test]
    fn a_nonzero_git_exit_is_ok_not_err() {
        let (host, _seen) = host_with_peer('/', |_| {
            Some((
                ControlReply::Ok(ReplyOk::Output(Output {
                    status: Some(128),
                    stdout: Vec::new(),
                    stderr: b"not a git repository".to_vec(),
                })),
                vec![],
            ))
        });
        let out = host.git(Path::new("/tmp"), &["rev-parse"]).unwrap();
        assert_eq!(out.status, Some(128));
        assert!(!out.success());
        assert_eq!(out.stderr, b"not a git repository");
    }

    #[test]
    fn the_host_id_is_derived_from_the_connection_and_is_stable() {
        let (a, _sa) = host_with_peer('/', |_| Some((ControlReply::Ok(ReplyOk::Unit), vec![])));
        let (b, _sb) = host_with_peer('/', |_| Some((ControlReply::Ok(ReplyOk::Unit), vec![])));
        assert_eq!(a.id(), b.id(), "same connection key, same id");
        assert_eq!(a.id(), a.id());
        assert_ne!(a.id(), HostId::LOCAL, "a remote host is never the local id");
        assert_eq!(a.id(), HostId::from_connection_key("ssh-alias:testbox"));
    }

    #[test]
    fn a_dead_link_reports_disconnected() {
        let (host, _seen) = host_with_peer('/', |_| None);
        let h: &dyn Host = host.as_ref();
        assert!(h.is_connected());
        let e = h.stat(Path::new("/f")).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::ConnectionReset);
        assert!(!h.is_connected());
    }

    #[test]
    fn a_reply_of_the_wrong_shape_is_invalid_data() {
        let (host, _seen) =
            host_with_peer('/', |_| Some((ControlReply::Ok(ReplyOk::Pong), vec![])));
        let e = host.stat(Path::new("/f")).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::InvalidData);
        assert!(e.to_string().contains("Pong"));
    }

    #[test]
    fn remote_host_is_object_safe() {
        let (host, _seen) =
            host_with_peer('/', |_| Some((ControlReply::Ok(ReplyOk::Unit), vec![])));
        let shared: SharedHost = host.into_shared();
        assert_eq!(shared.separator(), '/');
    }
}
