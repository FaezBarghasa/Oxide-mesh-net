//! Embedded network services models (MagicDNS, Oxide-Drop, Oxide-SSH, Oxide-Serve)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MagicDnsConfig {
    pub fqdn: String,
    pub aliases: Vec<String>,
    pub search_domains: Vec<String>,
    pub doh_upstream: String,
    pub is_resolution_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OxideDropTransfer {
    pub transfer_id: String,
    pub filename: String,
    pub file_size_bytes: u64,
    pub peer_id: String,
    pub peer_hostname: String,
    pub is_incoming: bool,
    pub progress_ratio: f32,
    pub speed_mbps: f32,
    pub blake3_verified: bool,
    pub is_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OxideSshSessionInfo {
    pub session_id: String,
    pub target_peer: String,
    pub cipher_suite: String,
    pub active_duration_secs: u64,
    pub is_recording: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IngressService {
    pub service_id: String,
    pub local_bind_addr: String,
    pub magic_dns_alias: String,
    pub is_public_funnel: bool,
    pub public_url: Option<String>,
    pub acme_tls_provisioned: bool,
    pub requests_handled: u64,
}
