//! Multi-queue virtual network interface for oxide-mesh-net

pub mod config;
pub mod error;
pub mod mss;
pub mod platform;

pub use config::{QueueConfig, RawFd, TunConfig};
pub use error::{Result, TunError};
pub use mss::{clamp_tcp_mss, update_checksum_16};
pub use platform::{QueueStats, TunDevice, TunQueue, create_tun};
