//! Platform-specific DNS interceptors and resolver integrators

pub mod resolv_conf;
pub mod systemd_resolved;

pub use resolv_conf::{ResolvConfConfig, ResolvConfManager};
pub use systemd_resolved::{SystemdResolvedConfig, SystemdResolvedController};
