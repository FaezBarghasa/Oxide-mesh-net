//! Zero-trust identity, RFC 8628 device flow, and ACL matrix models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceAuthSession {
    pub verification_uri: String,
    pub user_code: String,
    pub expires_in_secs: u32,
    pub is_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AclRule {
    pub id: String,
    pub src_tag: String,
    pub dst_tag: String,
    pub proto_mask: u8,
    pub port_range_start: u16,
    pub port_range_end: u16,
    pub action_allow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AclMatrixCell {
    pub src_tag: String,
    pub dst_tag: String,
    pub is_allowed: bool,
    pub rules_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardwareSecurityStatus {
    pub hsm_type: String,
    pub tpm2_bound: bool,
    pub cert_algo: String,
    pub cert_serial: String,
    pub expires_at: String,
    pub coordinator_signed: bool,
}
