//! Configuration management for oxide-daemon

use oxide_core::NodeId;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Static route configuration entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StaticRouteConfig {
    pub prefix: String,
    pub target_node: String,
    pub pmtu: Option<u16>,
}

/// Comprehensive configuration for oxide-daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub node_id: NodeId,
    pub mesh_name: String,
    pub coordinator_url: Option<String>,
    pub tun_name: String,
    pub mtu: u16,
    pub listen_port: u16,
    pub enable_dns: bool,
    pub enable_mss_clamp: bool,
    pub socket_path: Option<PathBuf>,
    #[serde(default = "default_overlay_ipv4")]
    pub overlay_ipv4: std::net::Ipv4Addr,
    #[serde(default = "default_overlay_ipv6")]
    pub overlay_ipv6: std::net::Ipv6Addr,
    #[serde(default)]
    pub static_routes: Vec<StaticRouteConfig>,
}

fn default_overlay_ipv4() -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::new(100, 64, 0, 42)
}

fn default_overlay_ipv6() -> std::net::Ipv6Addr {
    std::net::Ipv6Addr::new(0xfd00, 0x001d, 0x000e, 0, 0, 0, 0, 0x0042)
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            node_id: NodeId::new(),
            mesh_name: "default".into(),
            coordinator_url: None,
            tun_name: "oxide0".into(),
            mtu: 1420,
            listen_port: 51820,
            enable_dns: true,
            enable_mss_clamp: true,
            socket_path: None,
            overlay_ipv4: default_overlay_ipv4(),
            overlay_ipv6: default_overlay_ipv6(),
            static_routes: Vec::new(),
        }
    }
}

impl DaemonConfig {
    /// Load configuration from JSON file
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Save configuration to JSON file
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_config_serialization() {
        let config = DaemonConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DaemonConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.node_id, deserialized.node_id);
        assert_eq!(config.tun_name, deserialized.tun_name);
        assert_eq!(config.mtu, deserialized.mtu);
    }
}
