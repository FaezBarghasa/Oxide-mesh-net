//! Magicsock Multi-Path Endpoint Coordinator and Direct P2P Hole Punching

use super::stun::{NatType, StunClient};
use super::upnp::{UpnpClient, UpnpConfig};
use dashmap::DashMap;
use oxide_core::types::NodeId;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{debug, error, info, warn};

/// Connection path type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathType {
    DirectLocalLan,
    DirectStunP2P,
    DirectUpnpP2P,
    PortHoppedP2P,
    RelayedDerp,
    FallbackWss,
}

/// Discovered network candidate endpoint for a peer
#[derive(Debug, Clone, PartialEq)]
pub struct EndpointCandidate {
    pub addr: SocketAddr,
    pub path_type: PathType,
    pub latency: Option<Duration>,
    pub last_seen: Instant,
    pub priority: u32,
}

/// Magicsock NAT and Path Coordinator
pub struct MagicsockEngine {
    local_candidates: Vec<SocketAddr>,
    peer_candidates: Arc<DashMap<NodeId, Vec<EndpointCandidate>>>,
    active_paths: Arc<DashMap<NodeId, EndpointCandidate>>,
    stun_client: StunClient,
    upnp_client: UpnpClient,
    detected_nat_type: NatType,
}

impl MagicsockEngine {
    pub fn new(stun_servers: Vec<String>) -> Self {
        Self {
            local_candidates: Vec::new(),
            peer_candidates: Arc::new(DashMap::new()),
            active_paths: Arc::new(DashMap::new()),
            stun_client: StunClient::new(stun_servers),
            upnp_client: UpnpClient::new(UpnpConfig::default()),
            detected_nat_type: NatType::RestrictedCone,
        }
    }

    /// Register a discovered candidate endpoint for a peer
    pub fn add_peer_candidate(
        &self,
        node_id: NodeId,
        addr: SocketAddr,
        path_type: PathType,
        latency: Option<Duration>,
    ) {
        let candidate = EndpointCandidate {
            addr,
            path_type,
            latency,
            last_seen: Instant::now(),
            priority: match path_type {
                PathType::DirectLocalLan => 100,
                PathType::DirectUpnpP2P => 90,
                PathType::DirectStunP2P => 80,
                PathType::PortHoppedP2P => 70,
                PathType::RelayedDerp => 40,
                PathType::FallbackWss => 20,
            },
        };

        let mut list = self.peer_candidates.entry(node_id).or_default();
        // Update existing or append
        if let Some(existing) = list.iter_mut().find(|c| c.addr == addr) {
            existing.latency = latency;
            existing.last_seen = Instant::now();
        } else {
            list.push(candidate);
        }

        // Re-evaluate best path
        self.recalculate_best_path(node_id);
    }

    /// Recalculate best active path for a peer based on priority and latency
    pub fn recalculate_best_path(&self, node_id: NodeId) -> Option<EndpointCandidate> {
        let candidates = self.peer_candidates.get(&node_id)?;
        let best = candidates
            .iter()
            .min_by(|a, b| {
                // Higher priority first, then lower latency
                b.priority
                    .cmp(&a.priority)
                    .then_with(|| a.latency.cmp(&b.latency))
            })
            .cloned();

        if let Some(ref path) = best {
            self.active_paths.insert(node_id, path.clone());
        }
        best
    }

    /// Get current active path for a peer
    pub fn get_active_path(&self, node_id: &NodeId) -> Option<EndpointCandidate> {
        self.active_paths.get(node_id).map(|p| p.clone())
    }

    pub fn detected_nat_type(&self) -> NatType {
        self.detected_nat_type
    }

    pub fn set_detected_nat_type(&mut self, nat_type: NatType) {
        self.detected_nat_type = nat_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magicsock_path_ranking_and_failover() {
        let engine = MagicsockEngine::new(vec![]);
        let node_id = NodeId::new();

        let lan_addr: SocketAddr = "192.168.1.50:51820".parse().unwrap();
        let stun_addr: SocketAddr = "198.51.100.10:34182".parse().unwrap();
        let relay_addr: SocketAddr = "65.108.72.19:443".parse().unwrap();

        // 1. Add relay candidate
        engine.add_peer_candidate(
            node_id,
            relay_addr,
            PathType::RelayedDerp,
            Some(Duration::from_millis(50)),
        );
        let active = engine.get_active_path(&node_id).unwrap();
        assert_eq!(active.path_type, PathType::RelayedDerp);

        // 2. Add STUN P2P candidate -> should supersede relay
        engine.add_peer_candidate(
            node_id,
            stun_addr,
            PathType::DirectStunP2P,
            Some(Duration::from_millis(30)),
        );
        let active = engine.get_active_path(&node_id).unwrap();
        assert_eq!(active.path_type, PathType::DirectStunP2P);

        // 3. Add Local LAN candidate -> should supersede STUN
        engine.add_peer_candidate(
            node_id,
            lan_addr,
            PathType::DirectLocalLan,
            Some(Duration::from_millis(2)),
        );
        let active = engine.get_active_path(&node_id).unwrap();
        assert_eq!(active.path_type, PathType::DirectLocalLan);
        assert_eq!(active.addr, lan_addr);
    }
}
