//! Dynamic peer models and deep diagnostics

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerConnectionVector {
    DirectP2P,
    MasqueRelayed,
    FallbackWss,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerCardData {
    pub id: String,
    pub hostname: String,
    pub overlay_ip: String,
    pub os: String,
    pub connection_vector: PeerConnectionVector,
    pub remote_endpoint: String,
    pub rtt_ms: f32,
    pub last_handshake_secs: u64,
    pub bytes_rx: u64,
    pub bytes_tx: u64,
    pub is_online: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerDeepDiagnostics {
    pub peer_id: String,
    pub quic_version: String,
    pub congestion_algorithm: String,
    pub bbr_pacing_rate_mbps: f32,
    pub bbr_inflight_bytes: u32,
    pub rss_socket_pool_index: u8,
    pub public_key_ed25519: String,
    pub rekey_interval_secs: u32,
    pub tpm_endorsement_verified: bool,
    pub last_ping_rtt_us: Option<u32>,
}
