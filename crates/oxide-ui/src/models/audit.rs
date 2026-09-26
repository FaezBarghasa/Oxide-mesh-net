//! Cryptographic auditing, Merkle ledger, and packet stream models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MerkleAuditLogEntry {
    pub sequence_num: u64,
    pub timestamp: String,
    pub action_type: String,
    pub initiator_node_id: String,
    pub signature_hex: String,
    pub merkle_leaf_hash: String,
    pub root_hash_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PacketStreamEntry {
    pub timestamp_us: u64,
    pub src_ip: String,
    pub dst_ip: String,
    pub protocol: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub payload_bytes: usize,
    pub is_dropped: bool,
    pub drop_reason: Option<String>,
}
