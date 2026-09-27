pub mod error;
pub mod watchdog;

pub use error::{DnsError, Result};
pub use watchdog::{DnsWatchdog, DnsWatchdogConfig, OsDnsPlatform};