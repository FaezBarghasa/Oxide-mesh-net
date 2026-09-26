//! DPI circumvention, anti-censorship, and obfuscation parameters

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DpiObfuscationConfig {
    pub magic_header_scramble_enabled: bool,
    pub custom_magic_hex: String,
    pub padding_min_bytes: u16,
    pub padding_max_bytes: u16,
    pub pre_handshake_junk_enabled: bool,
    pub junk_burst_count: u8,
    pub junk_payload_max_bytes: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealityTlsConfig {
    pub enabled: bool,
    pub sni_target: String,
    pub server_public_key_hex: String,
    pub short_id_hex: String,
    pub diverted_probes_count: u64,
    pub remote_sni_reachable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientHelloFragConfig {
    pub enabled: bool,
    pub split_byte_offset: u16,
    pub split_delay_ms: u16,
    pub is_actively_splitting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortHoppingState {
    pub enabled: bool,
    pub port_range_start: u16,
    pub port_range_end: u16,
    pub hop_interval_secs: u32,
    pub current_port: u16,
    pub next_port_candidate: u16,
    pub seconds_to_next_hop: u32,
    pub emergency_wss_fallback_active: bool,
    pub enforce_wss_only: bool,
}
