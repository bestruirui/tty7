//! The mobile gateway, owned by the server.
//!
//! Phones reach the panes this server holds, so it keeps the gateway running
//! while its saved access switch is on. The server supplies the in-process
//! service; this module supervises it without depending on its implementation.
//!
//! A handoff stops the gateway before the exec ([`stop_for_handoff`]). The new
//! server restores it from the same configuration and state directory; a failed
//! handoff lets this server's supervisor start it again.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::core::config::config_dir_path;
use crate::daemon::control::{ControlRequest, ReplyOk};

/// The flag `tty7-gateway serve` takes to live exactly as long as its stdin.
pub const EXIT_WITH_STDIN_FLAG: &str = "--exit-with-stdin";

const STATUS_FILE: &str = "status.json";
/// How often the daemon looks at the switch and at its gateway.
const POLL: Duration = Duration::from_secs(2);
/// How long after a gateway start fails before trying again.
const RETRY: Duration = Duration::from_secs(30);

pub trait Gateway: Send {
    fn refresh(&mut self) -> Result<(), String>;
    fn stop(&mut self);
    fn request(&mut self, request: ControlRequest) -> std::io::Result<ReplyOk>;
}

/// The gateway service or its initialization error, shared by requests,
/// supervision and handoff.
static GATEWAY: Mutex<Option<Result<Box<dyn Gateway>, String>>> = Mutex::new(None);
/// Set by a handoff: from here to the exec, no new gateway is started.
static HANDING_OFF: AtomicBool = AtomicBool::new(false);

/// Stops the gateway and waits for it, before this process becomes another
/// program. The new image supervises afresh and starts its own.
pub fn stop_for_handoff() {
    HANDING_OFF.store(true, Ordering::SeqCst);
    if let Some(Ok(gateway)) = GATEWAY.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        gateway.stop();
    }
}

/// A handoff that did not happen: this daemon carries on, and its supervisor
/// starts the gateway again.
pub fn handoff_failed() {
    HANDING_OFF.store(false, Ordering::SeqCst);
}

pub fn request(request: ControlRequest) -> std::io::Result<ReplyOk> {
    let mut gateway = GATEWAY.lock().unwrap_or_else(|e| e.into_inner());
    if HANDING_OFF.load(Ordering::SeqCst) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "server is stopping",
        ));
    }
    match gateway.as_mut() {
        Some(Ok(gateway)) => gateway.request(request),
        Some(Err(error)) => Err(std::io::Error::other(error.clone())),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "mobile access service is not ready",
        )),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// The phone's iroh endpoint id — its public key, which iroh has already
    /// proven the peer holds by the time a stream reaches us.
    pub id: String,
    pub name: String,
    pub paired_at: u64,
}

/// The gateway's state directory: `<config dir>/mobile/`.
pub fn state_dir() -> Option<PathBuf> {
    Some(config_dir_path()?.join("mobile"))
}

/// The gateway's runtime status, returned to clients and persisted in
/// `mobile/status.json` for the standalone gateway command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Status {
    Running { id: String, since: u64 },
    Stopped,
    Failed { error: String },
}

impl Status {
    pub fn path_in(dir: &Path) -> PathBuf {
        dir.join(STATUS_FILE)
    }
}

/// Starts the thread that keeps a gateway running exactly while mobile
/// access is on. Called once by every daemon.
pub fn supervise(gateway: Result<Box<dyn Gateway>, String>) {
    let available = gateway.is_ok();
    if let Err(error) = &gateway {
        log::error!("mobile access unavailable: {error}");
    }
    *GATEWAY.lock().unwrap_or_else(|e| e.into_inner()) = Some(gateway);
    if !available {
        return;
    }
    let spawned = std::thread::Builder::new()
        .name("mobile-supervisor".into())
        .spawn(|| {
            let mut next_try = Instant::now();
            let mut noted = None;
            loop {
                let mut gateway = GATEWAY.lock().unwrap_or_else(|e| e.into_inner());
                if !HANDING_OFF.load(Ordering::SeqCst)
                    && Instant::now() >= next_try
                    && let Some(Ok(gateway)) = gateway.as_mut()
                {
                    match gateway.refresh() {
                        Ok(()) => {
                            noted = None;
                            next_try = Instant::now();
                        }
                        Err(e) => {
                            next_try = Instant::now() + RETRY;
                            if noted.as_ref() != Some(&e) {
                                log::warn!("mobile access: {e}");
                                noted = Some(e);
                            }
                        }
                    }
                }
                drop(gateway);
                std::thread::sleep(POLL);
            }
        });
    if let Err(e) = spawned {
        log::error!("could not start the mobile supervisor: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_status_file_keeps_its_shape() {
        // Written here and by tty7-gateway, read by the GUI: one shape.
        let json = serde_json::to_value(Status::Failed { error: "x".into() }).unwrap();
        assert_eq!(json, serde_json::json!({"state": "failed", "error": "x"}));
        let json = serde_json::to_value(Status::Running {
            id: "k".into(),
            since: 1,
        })
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!({"state": "running", "id": "k", "since": 1})
        );
    }
}
