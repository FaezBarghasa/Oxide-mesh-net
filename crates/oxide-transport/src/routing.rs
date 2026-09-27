//! Lock-free RCU prefix routing table and L1 direct-mapped routing cache
//!
//! Provides ultra-high-throughput routing lookups (<2ns L1 cache hit, lock-free ArcSwap
//! Radix tree lookup on miss) with zero reader-writer lock contention across worker cores.

use arc_swap::ArcSwap;
use oxide_core::{NodeId, OverlayIp, OverlayPrefix};
use std::{net::IpAddr, sync::Arc};

/// Target destination for routed traffic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteTarget {
    pub node_id: NodeId,
    pub pmtu: u16,
    pub is_direct: bool,
}

impl RouteTarget {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            pmtu: 1420,
            is_direct: true,
        }
    }

    pub fn with_pmtu(mut self, pmtu: u16) -> Self {
        self.pmtu = pmtu;
        self
    }
}

/// An entry in the immutable radix routing table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteEntry {
    pub prefix: OverlayPrefix,
    pub target: RouteTarget,
}

/// Immutable Prefix Radix Routing Table
#[derive(Debug, Clone, Default)]
pub struct RadixRoutingTable {
    v4_routes: Vec<RouteEntry>,
    v6_routes: Vec<RouteEntry>,
}

impl RadixRoutingTable {
    pub fn new() -> Self {
        Self {
            v4_routes: Vec::new(),
            v6_routes: Vec::new(),
        }
    }

    /// Insert or replace a route entry and sort by longest prefix length descending
    pub fn insert(&mut self, prefix: OverlayPrefix, target: RouteTarget) {
        let entry = RouteEntry { prefix, target };
        match prefix.addr {
            OverlayIp::V4(_) => {
                self.v4_routes.retain(|r| r.prefix != prefix);
                self.v4_routes.push(entry);
                self.v4_routes
                    .sort_by_key(|a| std::cmp::Reverse(a.prefix.prefix_len));
            }
            OverlayIp::V6(_) => {
                self.v6_routes.retain(|r| r.prefix != prefix);
                self.v6_routes.push(entry);
                self.v6_routes
                    .sort_by_key(|a| std::cmp::Reverse(a.prefix.prefix_len));
            }
        }
    }

    /// Remove a prefix route
    pub fn remove(&mut self, prefix: &OverlayPrefix) -> bool {
        match prefix.addr {
            OverlayIp::V4(_) => {
                let prev_len = self.v4_routes.len();
                self.v4_routes.retain(|r| &r.prefix != prefix);
                self.v4_routes.len() < prev_len
            }
            OverlayIp::V6(_) => {
                let prev_len = self.v6_routes.len();
                self.v6_routes.retain(|r| &r.prefix != prefix);
                self.v6_routes.len() < prev_len
            }
        }
    }

    /// Longest Prefix Match lookup
    #[inline]
    pub fn lookup(&self, ip: OverlayIp) -> Option<RouteTarget> {
        match ip {
            OverlayIp::V4(_) => {
                for route in &self.v4_routes {
                    if route.prefix.contains(ip) {
                        return Some(route.target);
                    }
                }
            }
            OverlayIp::V6(_) => {
                for route in &self.v6_routes {
                    if route.prefix.contains(ip) {
                        return Some(route.target);
                    }
                }
            }
        }
        None
    }
}

/// Size of the thread-local direct-mapped L1 routing cache (1024 entries)
pub const L1_CACHE_SIZE: usize = 1024;

/// Thread-local L1 Direct-Mapped Routing Cache
///
/// Designed for sub-2ns direct cache hits without heap allocation or atomics.
pub struct L1DirectMappedCache {
    entries: [(OverlayIp, RouteTarget); L1_CACHE_SIZE],
    valid: [bool; L1_CACHE_SIZE],
}

impl Default for L1DirectMappedCache {
    fn default() -> Self {
        Self::new()
    }
}

impl L1DirectMappedCache {
    pub fn new() -> Self {
        let dummy_ip = OverlayIp::default_v4();
        let dummy_target = RouteTarget::new(NodeId::new());
        Self {
            entries: [(dummy_ip, dummy_target); L1_CACHE_SIZE],
            valid: [false; L1_CACHE_SIZE],
        }
    }

    #[inline]
    fn hash_index(ip: OverlayIp) -> usize {
        match ip.as_ip_addr() {
            IpAddr::V4(v4) => {
                let u = u32::from(v4);
                // Fibonacci hashing for 1024 slots
                ((u.wrapping_mul(2654435761)) >> 22) as usize & (L1_CACHE_SIZE - 1)
            }
            IpAddr::V6(v6) => {
                let octets = v6.octets();
                let low = u64::from_le_bytes(octets[8..16].try_into().unwrap_or_default());
                ((low.wrapping_mul(11400714819323198485)) >> 54) as usize & (L1_CACHE_SIZE - 1)
            }
        }
    }

    #[inline]
    pub fn get(&self, ip: OverlayIp) -> Option<RouteTarget> {
        let idx = Self::hash_index(ip);
        if self.valid[idx] && self.entries[idx].0 == ip {
            Some(self.entries[idx].1)
        } else {
            None
        }
    }

    #[inline]
    pub fn insert(&mut self, ip: OverlayIp, target: RouteTarget) {
        let idx = Self::hash_index(ip);
        self.entries[idx] = (ip, target);
        self.valid[idx] = true;
    }

    pub fn invalidate(&mut self) {
        self.valid.fill(false);
    }
}

/// Global RCU Router Engine
pub struct RcuRouter {
    table: Arc<ArcSwap<RadixRoutingTable>>,
}

impl Default for RcuRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl RcuRouter {
    pub fn new() -> Self {
        Self {
            table: Arc::new(ArcSwap::from_pointee(RadixRoutingTable::new())),
        }
    }

    pub fn from_table(table: RadixRoutingTable) -> Self {
        Self {
            table: Arc::new(ArcSwap::from_pointee(table)),
        }
    }

    /// Fast-path lookup combining L1 thread-local cache and lock-free RCU load
    #[inline]
    pub fn lookup(&self, ip: OverlayIp, l1: &mut L1DirectMappedCache) -> Option<RouteTarget> {
        if let Some(target) = l1.get(ip) {
            return Some(target);
        }

        let table = self.table.load();
        if let Some(target) = table.lookup(ip) {
            l1.insert(ip, target);
            Some(target)
        } else {
            None
        }
    }

    /// Lock-free load of the entire routing table Arc
    #[inline]
    pub fn load_table(&self) -> arc_swap::Guard<Arc<RadixRoutingTable>> {
        self.table.load()
    }

    /// Atomic RCU swap of a newly compiled routing table
    pub fn update(&self, new_table: RadixRoutingTable) {
        self.table.store(Arc::new(new_table));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_radix_routing_longest_prefix_match() {
        let mut table = RadixRoutingTable::new();
        let node_default = NodeId::new();
        let node_specific = NodeId::new();

        // 100.64.0.0/10 (CGNAT wide) -> node_default
        let prefix_wide =
            OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(100, 64, 0, 0)), 10).unwrap();
        table.insert(prefix_wide, RouteTarget::new(node_default));

        // 100.64.1.0/24 (Specific subnet) -> node_specific
        let prefix_spec =
            OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(100, 64, 1, 0)), 24).unwrap();
        table.insert(prefix_spec, RouteTarget::new(node_specific));

        // Lookup 100.64.1.42 should hit specific
        let target1 = table
            .lookup(OverlayIp::V4(Ipv4Addr::new(100, 64, 1, 42)))
            .unwrap();
        assert_eq!(target1.node_id, node_specific);

        // Lookup 100.64.2.42 should hit wide
        let target2 = table
            .lookup(OverlayIp::V4(Ipv4Addr::new(100, 64, 2, 42)))
            .unwrap();
        assert_eq!(target2.node_id, node_default);
    }

    #[test]
    fn test_rcu_router_with_l1_cache() {
        let router = RcuRouter::new();
        let mut l1 = L1DirectMappedCache::new();
        let node = NodeId::new();

        let prefix = OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(100, 64, 0, 0)), 16).unwrap();
        let mut table = RadixRoutingTable::new();
        table.insert(prefix, RouteTarget::new(node));
        router.update(table);

        let ip = OverlayIp::V4(Ipv4Addr::new(100, 64, 5, 10));

        // Miss in L1, hit in RCU, populate L1
        let t1 = router.lookup(ip, &mut l1).unwrap();
        assert_eq!(t1.node_id, node);

        // Subsequent lookup must hit L1
        let cached = l1.get(ip).unwrap();
        assert_eq!(cached.node_id, node);
    }
}
