mod boxdraw;
mod color_scheme;
mod command_cursor;
mod composer;
pub mod element;
pub mod fps;
pub(crate) mod git_status;
pub(crate) mod images;
pub mod input;
mod loopback;
pub mod palette;
pub(crate) mod pane_liveness;
pub(crate) mod parked_cursor;
pub(crate) mod prompt_reflow;
mod remote;
pub(crate) mod scrollbar;
pub mod search;
mod size;
mod smart_select;
pub mod view;

#[cfg(all(test, target_os = "windows"))]
pub(crate) use remote::notify_desktop_for_pane;
#[cfg(target_os = "windows")]
pub(crate) use remote::toast_activated;
pub use remote::{PaneRoute, PaneWorkspace, RemoteTerminal, attach_refused, attach_unanswered};
pub use size::TermSize;
