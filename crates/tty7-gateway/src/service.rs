//! The gateway as a service a process can start and stop: the `tty7-gateway
//! serve` command runs one until it is killed, and the server runs one for as
//! long as mobile access is switched on.
//!
//! The server owns the state in `<config dir>/mobile/` and handles the GUI's
//! control requests. The standalone command also uses these files, including
//! `status.json` for the gateway's current state.

use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::mpsc as std_mpsc;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use iroh::endpoint::{BindOpts, presets};
use iroh::{Endpoint, SecretKey, Watcher as _};
use iroh_mdns_address_lookup::MdnsAddressLookup;
use tokio::sync::oneshot;
use tty7_core::daemon::control::{ControlRequest, ReplyOk};
use tty7_mobile_proto::{ALPN, MDNS_SERVICE, PairCode};

use crate::daemon::Daemon;
use crate::identity;
use crate::route::{self, Route};
use crate::serve::{self, Backend};
use crate::state::{Reachable, State, Status, failed, running};

/// How long a relay may stay out of reach before the gateway tries other ways
/// out to it. Long enough for a healthy one to connect, which takes a second
/// or two.
const RELAY_WAIT: Duration = Duration::from_secs(10);
/// How often, while no way out reaches a relay, the gateway tries them again.
const RELAY_RETRY: Duration = Duration::from_secs(60);
/// How often the relay's state is looked at.
const RELAY_POLL: Duration = Duration::from_secs(2);
/// How long a way out being tried has to reach a relay.
const PROBE_WAIT: Duration = Duration::from_secs(8);
const PROBE_POLL: Duration = Duration::from_millis(250);

/// A gateway running on its own thread. Dropping it stops it.
pub struct Running {
    stop: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
    status: Arc<Mutex<Status>>,
}

pub struct Manager {
    state: State,
    /// The saved switch, kept when the running service stops for a handoff.
    enabled: bool,
    backend: Arc<dyn Backend>,
    running: Option<Running>,
    error: Option<String>,
}

impl Manager {
    pub fn new(backend: Arc<dyn Backend>) -> Result<Self> {
        let state = State::open_default()?;
        Ok(Self {
            enabled: state.enabled()?,
            state,
            backend,
            running: None,
            error: None,
        })
    }

    fn status(&self) -> Status {
        if let Some(error) = &self.error {
            return Status::Failed {
                error: error.clone(),
            };
        }
        match self.running.as_ref() {
            Some(running) if !running.thread.as_ref().is_some_and(JoinHandle::is_finished) => {
                running
                    .status
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .clone()
            }
            Some(running) => match running
                .status
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone()
            {
                status @ Status::Failed { .. } => status,
                _ => Status::Failed {
                    error: "mobile gateway stopped unexpectedly".into(),
                },
            },
            None => Status::Stopped,
        }
    }
}

impl tty7_core::daemon::mobile::Gateway for Manager {
    fn refresh(&mut self) -> std::result::Result<(), String> {
        if !self.enabled {
            self.stop();
            return Ok(());
        }
        if self
            .running
            .as_ref()
            .is_some_and(|r| r.thread.as_ref().is_some_and(JoinHandle::is_finished))
        {
            self.running.take();
        }
        if self.running.is_none() {
            match start_with(self.state.clone(), self.backend.clone()) {
                Ok(started) => {
                    self.running = Some(started);
                    self.error = None;
                }
                Err(e) => {
                    self.error = Some(format!("{e:#}"));
                    return Err(format!("{e:#}"));
                }
            }
        }
        Ok(())
    }

    fn stop(&mut self) {
        self.running.take();
        self.error = None;
    }

    fn request(&mut self, request: ControlRequest) -> io::Result<ReplyOk> {
        match request {
            ControlRequest::MobileStatus | ControlRequest::MobileSetEnabled { .. } => {
                if let ControlRequest::MobileSetEnabled { enabled } = request {
                    self.state.set_enabled(enabled).map_err(io::Error::other)?;
                    self.enabled = enabled;
                    self.refresh().map_err(io::Error::other)?;
                }
                Ok(ReplyOk::MobileStatus {
                    enabled: self.enabled,
                    status: self.status(),
                    devices: self.state.devices().map_err(io::Error::other)?,
                })
            }
            ControlRequest::MobilePair => {
                if !matches!(self.status(), Status::Running { .. }) {
                    return Err(io::Error::new(
                        io::ErrorKind::NotConnected,
                        "mobile access is not running",
                    ));
                }
                let devices = self.state.devices().map_err(io::Error::other)?;
                let offer =
                    pair_code(&self.state, Duration::from_secs(600)).map_err(io::Error::other)?;
                Ok(ReplyOk::MobilePair {
                    code: offer.code,
                    secret: offer.secret,
                    devices,
                })
            }
            ControlRequest::MobileCancelPair { secret } => {
                self.state
                    .close_pairing(&secret)
                    .map_err(io::Error::other)?;
                Ok(ReplyOk::Unit)
            }
            ControlRequest::MobilePairing { secret } => Ok(ReplyOk::MobilePairing {
                open: self.state.pairing_is_open(&secret),
                has_open: self.state.has_open_pairing(),
            }),
            ControlRequest::MobileRevoke { id } => {
                self.state.revoke(&id).map_err(io::Error::other)?;
                Ok(ReplyOk::MobileDevices(
                    self.state.devices().map_err(io::Error::other)?,
                ))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "not a mobile access request",
            )),
        }
    }
}

impl Running {
    /// Stops accepting phones, closes every connection, and waits for the
    /// thread to finish.
    pub fn stop(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Starts the gateway on a thread of its own and returns once it is listening
/// — or with why it could not: another gateway holds this config dir, or no
/// socket could be bound.
pub fn start(state: State) -> Result<Running> {
    start_with(state, Arc::new(Daemon::default()))
}

fn start_with(state: State, backend: Arc<dyn Backend>) -> Result<Running> {
    let (ready_tx, ready_rx) = std_mpsc::channel::<Result<()>>();
    let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let status = Arc::new(Mutex::new(Status::Stopped));
    let thread_status = status.clone();
    let thread = std::thread::Builder::new()
        .name("mobile-gateway".into())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("mobile-gateway-rt")
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(e) => {
                    let _ = ready_tx.send(Err(e.into()));
                    return;
                }
            };
            runtime.block_on(run(
                state,
                backend.clone(),
                ready_tx,
                stop_rx,
                thread_status,
            ));
            drop(runtime);
            backend.shutdown();
        })
        .context("starting the gateway thread")?;
    match ready_rx.recv() {
        Ok(Ok(())) => Ok(Running {
            stop: Some(stop_tx),
            thread: Some(thread),
            status,
        }),
        Ok(Err(e)) => {
            let _ = thread.join();
            Err(e)
        }
        Err(_) => {
            let _ = thread.join();
            anyhow::bail!("the gateway thread ended before it was listening")
        }
    }
}

async fn run(
    state: State,
    backend: Arc<dyn Backend>,
    ready: std_mpsc::Sender<Result<()>>,
    mut stop: oneshot::Receiver<()>,
    status: Arc<Mutex<Status>>,
) {
    // Another gateway already serving this config dir owns `status.json`; one
    // that could not take the lock must not overwrite what it says.
    let lock = match state.lock_serve() {
        Ok(lock) => lock,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    // The most likely way out, taken without waiting to see: phones on this
    // network need none, and a better one is found while they are served.
    let mut route = routes().await.into_iter().next().unwrap_or(Route::Direct);
    let started = async {
        let key = state.secret_key()?;
        let endpoint = bind(key.clone(), state.port(), &route).await?;
        remember_port(&endpoint, &state)?;
        anyhow::Ok((key, endpoint))
    };
    let (key, mut endpoint) = match started.await {
        Ok(up) => up,
        Err(e) => {
            let _ = state.set_status(&failed(&e));
            let _ = ready.send(Err(e));
            return;
        }
    };
    let id = endpoint.id().to_string();
    log::info!("mobile gateway listening as {id} (relay: {route})");
    let initial_status = running(&id);
    *status.lock().unwrap_or_else(|e| e.into_inner()) = initial_status.clone();
    let _ = state.set_status(&initial_status);
    let _ = ready.send(Ok(()));

    // Not fatal: a standalone gateway may start before its server, and phones
    // are told the same thing until that server is available.
    if let Err(e) = backend.snapshot() {
        log::warn!("{}", serve::server_down(&backend.hostname(), &e));
    }

    loop {
        let addrs = tokio::spawn(keep_reachable(endpoint.clone(), state.clone()));
        let switch = tokio::select! {
            () = serve::run(endpoint.clone(), state.clone(), backend.clone()) => None,
            _ = &mut stop => None,
            better = better_route(&endpoint, &route) => Some(better),
        };
        addrs.abort();
        let _ = addrs.await;
        let Some(better) = switch else { break };
        // iroh takes its proxy when an endpoint is built, so a new way out is
        // a new endpoint: the same key on the same port, so phones' pairing
        // codes still reach it. Only a relay that has stayed out of reach gets
        // here, so what this drops is at most connections on this network,
        // which the phone makes again.
        log::info!("mobile gateway: its relay answers {better}, not {route} — switching");
        // Every handle on the old endpoint has to be gone before its port is
        // free to bind again.
        endpoint.close().await;
        drop(endpoint);
        let rebound = async {
            let endpoint = bind(key.clone(), state.port(), &better).await?;
            remember_port(&endpoint, &state)?;
            anyhow::Ok(endpoint)
        };
        match rebound.await {
            Ok(new) => {
                endpoint = new;
                route = better;
            }
            Err(e) => {
                log::warn!("mobile gateway: {e:#}");
                *status.lock().unwrap_or_else(|e| e.into_inner()) = failed(&e);
                let _ = state.set_status(&failed(&e));
                drop(lock);
                return;
            }
        }
    }
    endpoint.close().await;
    *status.lock().unwrap_or_else(|e| e.into_inner()) = Status::Stopped;
    let _ = state.set_status(&Status::Stopped);
    drop(lock);
    log::info!("mobile gateway stopped");
}

/// Keeps the port a restart should bind again, so the addresses in phones'
/// pairing codes still reach this machine.
fn remember_port(endpoint: &Endpoint, state: &State) -> Result<()> {
    if let Some(port) = endpoint
        .bound_sockets()
        .iter()
        .map(SocketAddr::port)
        .find(|&p| p != 0)
        && state.port() != Some(port)
    {
        state.set_port(port).context("remembering the port")?;
    }
    Ok(())
}

/// Keeps the addresses a pairing code carries current. The local ones are
/// known at once and are all a phone on the same network needs. The relay is
/// only written down once it is connected: the one iroh picks by latency is
/// not necessarily one it can reach, and a code naming it sends a phone
/// somewhere this machine is not.
async fn keep_reachable(endpoint: Endpoint, state: State) {
    loop {
        let reachable = Reachable {
            relay: connected_relay(&endpoint),
            addrs: endpoint.addr().ip_addrs().map(|a| a.to_string()).collect(),
        };
        if state.reachable() != reachable {
            let _ = state.set_reachable(&reachable);
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn connected_relay(endpoint: &Endpoint) -> Option<String> {
    endpoint
        .home_relay_status()
        .get()
        .iter()
        .find(|status| status.is_connected())
        .map(|status| status.url().to_string())
}

/// Every way out to try, most likely first. Reading them touches the config
/// file and, on macOS, the system configuration store.
async fn routes() -> Vec<Route> {
    tokio::task::spawn_blocking(route::routes)
        .await
        .unwrap_or_else(|_| vec![Route::Direct])
}

/// Resolves, never, while `endpoint`'s relay is up. Once it has been out of
/// reach for a while, tries every other way out on an endpoint of its own and
/// resolves with the first that reaches a relay — and if none does, looks
/// again every so often: a proxy is started, a network changes.
async fn better_route(endpoint: &Endpoint, current: &Route) -> Route {
    let mut down_for = RELAY_WAIT;
    let mut said = false;
    loop {
        relay_down_for(endpoint, down_for).await;
        for route in routes().await.into_iter().filter(|r| r != current) {
            if reaches_relay(&route).await {
                return route;
            }
        }
        if !said {
            log::warn!(
                "mobile gateway: no relay answers ({current}, or any proxy tty7 knows of) — \
                 phones on other networks cannot reach this machine"
            );
            said = true;
        }
        down_for = RELAY_RETRY;
    }
}

/// Resolves once `endpoint` has gone `span` without a connected relay.
async fn relay_down_for(endpoint: &Endpoint, span: Duration) {
    let mut since = Instant::now();
    loop {
        tokio::time::sleep(RELAY_POLL).await;
        if connected_relay(endpoint).is_some() {
            since = Instant::now();
        } else if since.elapsed() >= span {
            return;
        }
    }
}

/// Whether a relay answers by `route`, asked on a throwaway endpoint so the
/// one phones use is not disturbed.
async fn reaches_relay(route: &Route) -> bool {
    let mut builder = Endpoint::builder(presets::N0).clear_address_lookup();
    if let Some(proxy) = route.proxy() {
        builder = builder.proxy_url(proxy.clone());
    }
    let Ok(probe) = builder.bind().await else {
        return false;
    };
    let reached = tokio::time::timeout(PROBE_WAIT, async {
        while connected_relay(&probe).is_none() {
            tokio::time::sleep(PROBE_POLL).await;
        }
    })
    .await
    .is_ok();
    probe.close().await;
    reached
}

/// Binds the gateway's endpoint on the port it had last time, so the
/// addresses in phones' pairing codes still reach it, and advertises it on the
/// local network. Each of those is given up, with a note, rather than let it
/// keep the gateway from starting: the port may be taken, and multicast may be
/// off. Its relay is reached by `route`.
async fn bind(key: SecretKey, port: Option<u16>, route: &Route) -> Result<Endpoint> {
    let mut attempts = Vec::new();
    if let Some(port) = port {
        attempts.extend([(port, true), (port, false)]);
    }
    attempts.extend([(0, true), (0, false)]);

    let mut failures = Vec::new();
    for (port, mdns) in attempts {
        let mut builder = Endpoint::builder(presets::N0)
            .secret_key(key.clone())
            .alpns(vec![ALPN.to_vec()])
            .clear_ip_transports()
            .bind_addr(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))?
            // IPv6 is a bonus, as it is in iroh's own defaults.
            .bind_addr_with_opts(
                SocketAddr::from((Ipv6Addr::UNSPECIFIED, port)),
                BindOpts::default().set_is_required(false),
            )?;
        if let Some(proxy) = route.proxy() {
            builder = builder.proxy_url(proxy.clone());
        }
        let builder = match mdns {
            true => builder.address_lookup(MdnsAddressLookup::builder().service_name(MDNS_SERVICE)),
            false => builder,
        };
        match builder.bind().await {
            Ok(endpoint) => {
                for failure in failures {
                    log::warn!("mobile gateway: {failure} — carrying on without it");
                }
                return Ok(endpoint);
            }
            Err(e) => {
                let what = match (port, mdns) {
                    (0, true) => "local network discovery".to_string(),
                    (0, false) => "the connection".to_string(),
                    (port, _) => format!("port {port}"),
                };
                failures.push(format!("{what}: {e}"));
            }
        }
    }
    anyhow::bail!("could not start: {}", failures.join("; "))
}

/// Serves until this process's stdin closes. A caller holding the other end of
/// the pipe controls the standalone gateway's lifetime.
pub fn serve_until_stdin_closes(state: State) -> Result<()> {
    let running = start(state)?;
    let mut sink = [0u8; 256];
    let mut stdin = std::io::stdin().lock();
    while matches!(std::io::Read::read(&mut stdin, &mut sink), Ok(n) if n > 0) {}
    running.stop();
    Ok(())
}

/// An open pairing offer: the code a phone scans or pastes, and the secret in
/// it, which [`State::pairing_is_open`] and [`State::close_pairing`] take.
pub struct PairOffer {
    pub code: String,
    pub secret: String,
}

/// Opens a pairing offer valid for `ttl`, replacing any earlier one. The
/// addresses in its code are the ones the running gateway last wrote down;
/// with none running the code still pairs once one starts.
pub fn pair_code(state: &State, ttl: Duration) -> Result<PairOffer> {
    let reachable = state.reachable();
    let secret = state.open_pairing(ttl.as_secs())?;
    let code = PairCode {
        host_id: state.secret_key()?.public().to_string(),
        host_name: identity::name(),
        relay: reachable.relay,
        addrs: reachable.addrs,
        secret: secret.clone(),
        machine: identity::fingerprint(),
    }
    .encode();
    Ok(PairOffer { code, secret })
}
