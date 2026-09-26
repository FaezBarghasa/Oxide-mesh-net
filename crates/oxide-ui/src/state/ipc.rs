//! Local IPC and Actix Web WebSocket transport layer for oxide-ui

use crate::models::DaemonStateDelta;

pub struct IpcBridgeConfig {
    pub ws_endpoint: String,
    pub token: Option<String>,
}

impl Default for IpcBridgeConfig {
    fn default() -> Self {
        Self {
            ws_endpoint: "ws://127.0.0.1:9090/api/v1/ws/control".to_string(),
            token: None,
        }
    }
}

pub struct StateReconciler;

impl StateReconciler {
    pub fn decode_delta_postcard(raw_bytes: &[u8]) -> Result<DaemonStateDelta, postcard::Error> {
        postcard::from_bytes(raw_bytes)
    }

    pub fn decode_delta_json(json_str: &str) -> Result<DaemonStateDelta, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn compute_backoff_delay_ms(attempt: u32) -> u64 {
        let base = 200u64;
        let max = 5000u64;
        let exp = 2u64.saturating_pow(attempt.min(6));
        (base * exp).min(max)
    }
}
