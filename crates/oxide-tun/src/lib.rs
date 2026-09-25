//! Multi-queue virtual network interface for oxide-mesh-net

pub mod config;
pub mod error;
pub mod platform;

pub use config::{TunConfig, QueueConfig, RawFd};
pub use error::{TunError, Result};
pub use platform::{create_tun, TunDevice, TunQueue, QueueStats};