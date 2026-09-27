//! TUN interface configuration

use crate::error::Result;
use ipnet::IpNet;

/// TUN interface configuration
#[derive(Debug, Clone)]
pub struct TunConfig {
    /// Interface name (e.g., "oxide0", "utun0")
    pub name: String,
    /// Interface MTU
    pub mtu: u16,
    /// Number of queues (for multi-queue support)
    pub num_queues: usize,
    /// IPv4 address and prefix
    pub ipv4: Option<IpNet>,
    /// IPv6 address and prefix
    pub ipv6: Option<IpNet>,
    /// Whether to bring interface up automatically
    pub up: bool,
    /// Whether to enable GRO/GSO offload
    pub offload: bool,
    /// Whether this is a persistent interface
    pub persistent: bool,
    /// Owner user ID (Linux only)
    pub owner_uid: Option<u32>,
    /// Owner group ID (Linux only)
    pub owner_gid: Option<u32>,
}

impl Default for TunConfig {
    fn default() -> Self {
        Self {
            name: "oxide0".into(),
            mtu: 1500,
            num_queues: 1,
            ipv4: None,
            ipv6: None,
            up: true,
            offload: true,
            persistent: false,
            owner_uid: None,
            owner_gid: None,
        }
    }
}

impl TunConfig {
    /// Create a new TUN config with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set number of queues
    pub fn with_queues(mut self, num_queues: usize) -> Result<Self> {
        if num_queues == 0 {
            return Err(crate::error::TunError::InvalidConfig(
                "Number of queues must be > 0".into(),
            ));
        }
        self.num_queues = num_queues;
        Ok(self)
    }

    /// Set MTU
    pub fn with_mtu(mut self, mtu: u16) -> Result<Self> {
        if mtu < 68 {
            return Err(crate::error::TunError::InvalidConfig(
                "MTU must be at least 68".into(),
            ));
        }
        self.mtu = mtu;
        Ok(self)
    }

    /// Set IPv4 address
    pub fn with_ipv4(mut self, ipv4: IpNet) -> Result<Self> {
        if !matches!(ipv4, IpNet::V4(_)) {
            return Err(crate::error::TunError::InvalidConfig(
                "IPv4 address required".into(),
            ));
        }
        self.ipv4 = Some(ipv4);
        Ok(self)
    }

    /// Set IPv6 address
    pub fn with_ipv6(mut self, ipv6: IpNet) -> Result<Self> {
        if !matches!(ipv6, IpNet::V6(_)) {
            return Err(crate::error::TunError::InvalidConfig(
                "IPv6 address required".into(),
            ));
        }
        self.ipv6 = Some(ipv6);
        Ok(self)
    }

    /// Enable/disable interface up
    pub fn with_up(mut self, up: bool) -> Self {
        self.up = up;
        self
    }

    /// Enable/disable offload
    pub fn with_offload(mut self, offload: bool) -> Self {
        self.offload = offload;
        self
    }

    /// Set persistent mode
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    /// Set owner (Linux only)
    pub fn with_owner(mut self, uid: Option<u32>, gid: Option<u32>) -> Self {
        self.owner_uid = uid;
        self.owner_gid = gid;
        self
    }
}

/// Queue configuration for multi-queue TUN
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// Queue index
    pub queue_id: usize,
    /// File descriptor / handle for this queue
    pub fd: RawFd,
    /// Whether this queue is for RX
    pub rx: bool,
    /// Whether this queue is for TX
    pub tx: bool,
}

/// Raw file descriptor type (platform-specific)
#[cfg(target_os = "linux")]
pub type RawFd = std::os::fd::RawFd;

#[cfg(target_os = "windows")]
pub type RawFd = std::os::windows::io::RawHandle;

#[cfg(target_os = "macos")]
pub type RawFd = std::os::fd::RawFd;
