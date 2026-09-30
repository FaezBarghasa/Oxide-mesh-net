pub mod error;
pub mod platform;
pub mod server;
pub mod watchdog;

pub use error::{DnsError, Result};
pub use platform::{
    ResolvConfConfig, ResolvConfManager, SystemdResolvedConfig, SystemdResolvedController,
};
pub use server::{MagicDnsConfig, MagicDnsServer, MeshHostEntry};
pub use watchdog::{DnsWatchdog, DnsWatchdogConfig, OsDnsPlatform};
