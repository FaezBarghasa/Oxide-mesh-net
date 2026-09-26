//! State delta framing and core lifecycle models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Connected,
    Connecting,
    Disconnecting,
    Offline,
    EmergencyObfuscation,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NatType {
    FullCone,
    RestrictedCone,
    PortRestricted,
    Symmetric,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMode {
    DirectUdp,
    MasqueRelay,
    EmergencyWss,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub ingress_bytes_sec: u64,
    pub egress_bytes_sec: u64,
    pub pps_in: u32,
    pub pps_out: u32,
    pub current_mtu: u16,
    pub overlay_ipv4: String,
    pub overlay_ipv6: String,
    pub active_threads: u32,
    pub nat_type: NatType,
    pub throughput_history: Vec<f64>,
    pub latency_history: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaemonStateDelta {
    pub timestamp_ms: u64,
    pub node_state: NodeState,
    pub telemetry: TelemetrySnapshot,
    pub active_peer_count: usize,
    pub active_exit_node: Option<String>,
    pub emergency_transport_active: bool,
}
