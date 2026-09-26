//! Routing fabric, exit node, and subnet governance models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExitNodeOption {
    pub id: String,
    pub hostname: String,
    pub location_country: String,
    pub location_city: String,
    pub latency_ms: f32,
    pub is_active: bool,
    pub allows_lan_access: bool,
    pub verified_public_ip: Option<String>,
    pub dns_leak_protected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdvertisedSubnet {
    pub cidr: String,
    pub interface_name: String,
    pub is_active: bool,
    pub has_conflict: bool,
    pub active_vrrp_leader: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SplitRouteRule {
    pub id: String,
    pub pattern: String,
    pub bypass_mesh: bool,
    pub target_app_process: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteDecisionTestResult {
    pub query: String,
    pub decision: String,
    pub matching_rule: Option<String>,
    pub egress_interface: String,
}
