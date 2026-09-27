//! Multi-queue virtual network interface for oxide-mesh-net

pub mod config;
pub mod error;
pub mod mss;
pub mod platform;

pub use config::{TunConfig, QueueConfig, RawFd};
pub use error::{TunError, Result};
pub use mss::{clamp_tcp_mss, update_checksum_16};
pub use platform::{create_tun, TunDevice, TunQueue, QueueStats};