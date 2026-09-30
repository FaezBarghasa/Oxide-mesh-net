//! MagicDNS local resolver server and mesh domain name service

use dashmap::DashMap;
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::UdpSocket;
use tracing::{debug, error, info, warn};

/// MagicDNS server configuration
#[derive(Debug, Clone)]
pub struct MagicDnsConfig {
    pub bind_addr: SocketAddr,
    pub mesh_domain: String,
    pub upstream_dns: Vec<SocketAddr>,
}

impl Default for MagicDnsConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:5353".parse().unwrap(),
            mesh_domain: "mesh.oxide".to_string(),
            upstream_dns: vec![
                "1.1.1.1:53".parse().unwrap(),
                "8.8.8.8:53".parse().unwrap(),
            ],
        }
    }
}

/// Mesh host entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshHostEntry {
    pub hostname: String,
    pub ipv4: Ipv4Addr,
    pub ipv6: Option<Ipv6Addr>,
}

/// MagicDNS local server and cache
pub struct MagicDnsServer {
    config: MagicDnsConfig,
    hosts: Arc<DashMap<String, MeshHostEntry>>,
    reverse_v4: Arc<DashMap<Ipv4Addr, String>>,
}

impl MagicDnsServer {
    pub fn new(config: MagicDnsConfig) -> Self {
        Self {
            config,
            hosts: Arc::new(DashMap::new()),
            reverse_v4: Arc::new(DashMap::new()),
        }
    }

    /// Register or update a mesh peer in MagicDNS
    pub fn register_peer(&self, hostname: &str, ipv4: Ipv4Addr, ipv6: Option<Ipv6Addr>) {
        let fqdn = if hostname.ends_with(&self.config.mesh_domain) {
            hostname.to_lowercase()
        } else {
            format!("{}.{}", hostname, self.config.mesh_domain).to_lowercase()
        };

        let entry = MeshHostEntry {
            hostname: fqdn.clone(),
            ipv4,
            ipv6,
        };

        self.hosts.insert(fqdn.clone(), entry);
        self.reverse_v4.insert(ipv4, fqdn);
    }

    /// Remove a mesh peer from MagicDNS
    pub fn unregister_peer(&self, hostname: &str) {
        let fqdn = if hostname.ends_with(&self.config.mesh_domain) {
            hostname.to_lowercase()
        } else {
            format!("{}.{}", hostname, self.config.mesh_domain).to_lowercase()
        };

        if let Some((_, entry)) = self.hosts.remove(&fqdn) {
            self.reverse_v4.remove(&entry.ipv4);
        }
    }

    /// Lookup an A record (IPv4)
    pub fn lookup_a(&self, query_domain: &str) -> Option<Ipv4Addr> {
        let normalized = query_domain.trim_end_matches('.').to_lowercase();
        self.hosts.get(&normalized).map(|h| h.ipv4)
    }

    /// Lookup an AAAA record (IPv6)
    pub fn lookup_aaaa(&self, query_domain: &str) -> Option<Ipv6Addr> {
        let normalized = query_domain.trim_end_matches('.').to_lowercase();
        self.hosts.get(&normalized).and_then(|h| h.ipv6)
    }

    /// Reverse lookup (PTR)
    pub fn lookup_ptr(&self, ip: &Ipv4Addr) -> Option<String> {
        self.reverse_v4.get(ip).map(|v| v.clone())
    }

    /// Total registered hosts count
    pub fn host_count(&self) -> usize {
        self.hosts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_dns_registration_and_lookup() {
        let server = MagicDnsServer::new(MagicDnsConfig::default());
        assert_eq!(server.host_count(), 0);

        let ip4 = Ipv4Addr::new(100, 64, 0, 42);
        let ip6 = "fd00:0x1d:e::42".parse().ok();

        server.register_peer("workstation", ip4, ip6);
        assert_eq!(server.host_count(), 1);

        // Lookup with and without domain suffix
        assert_eq!(server.lookup_a("workstation.mesh.oxide"), Some(ip4));
        assert_eq!(server.lookup_a("WORKSTATION.MESH.OXIDE."), Some(ip4));
        assert_eq!(server.lookup_aaaa("workstation.mesh.oxide"), ip6);

        // Reverse PTR
        assert_eq!(
            server.lookup_ptr(&ip4),
            Some("workstation.mesh.oxide".to_string())
        );

        // Unregister
        server.unregister_peer("workstation");
        assert_eq!(server.host_count(), 0);
        assert_eq!(server.lookup_a("workstation.mesh.oxide"), None);
    }
}
