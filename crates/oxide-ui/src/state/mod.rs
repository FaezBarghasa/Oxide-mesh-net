//! State and transport modules for oxide-ui

pub mod app_state;
pub mod ipc;

pub use app_state::{ActiveView, AppState};
pub use ipc::{IpcBridgeConfig, StateReconciler};
