use serde::{Deserialize, Serialize};
use oxide_core::{NodeId, MeshName, OverlayPrefix, OverlayIp, Endpoint, NodeCapabilities};
use oxide_crypto::keys::{DeviceIdentityPublicKey, SessionPublicKey, KeyFingerprint};
pub use crate::wire::{AclAction, AclDirection};

/// Topic namespace prefix
pub const TOPIC_ROOT: &str = "oxide";

/// Topic separators
pub const TOPIC_SEP: &str = "/";

/// QoS levels
pub const QOS_AT_MOST_ONCE: u8 = 0;    // Fire and forget
pub const QOS_AT_LEAST_ONCE: u8 = 1;   // Acknowledged
pub const QOS_EXACTLY_ONCE: u8 = 2;    // Assured

/// Retained message flag
pub const RETAINED: bool = true;
pub const NOT_RETAINED: bool = false;

/// Build a topic string
pub fn topic(parts: &[&str]) -> String {
    let mut all = Vec::with_capacity(1 + parts.len());
    all.push(TOPIC_ROOT);
    all.extend_from_slice(parts);
    all.join(TOPIC_SEP)
}

/// Mesh-scoped topic
pub fn mesh_topic(mesh: &MeshName, parts: &[&str]) -> String {
    let mut all = Vec::with_capacity(1 + parts.len());
    all.push(mesh.as_str());
    all.extend_from_slice(parts);
    topic(&all)
}

/// Node-scoped topic
pub fn node_topic(mesh: &MeshName, node: &NodeId, parts: &[&str]) -> String {
    let node_str = node.to_string();
    let mut all = Vec::with_capacity(1 + parts.len());
    all.push(node_str.as_str());
    all.extend_from_slice(parts);
    mesh_topic(mesh, &all)
}

/// Topic patterns for subscription
pub struct TopicPatterns;

impl TopicPatterns {
    /// All mesh events (for coordinators)
    pub fn all_mesh_events(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["#"])
    }

    /// All node events in mesh
    pub fn all_node_events(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["+", "#"])
    }

    /// Specific node events
    pub fn node_events(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["#"])
    }
}

/// Node presence topics (retained)
pub struct PresenceTopics;

impl PresenceTopics {
    /// Announce node presence (retained)
    pub fn announce(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["presence", "announce"])
    }

    /// Node heartbeat (not retained, high frequency)
    pub fn heartbeat(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["presence", "heartbeat"])
    }

    /// Last Will Testament (set on connect, published on ungraceful disconnect)
    pub fn lwt(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["presence", "lwt"])
    }

    /// Node metadata (retained, updated on config change)
    pub fn metadata(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["presence", "metadata"])
    }
}

/// Route advertisement topics (retained)
pub struct RouteTopics;

impl RouteTopics {
    /// Advertise route (retained)
    pub fn advertise(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["routes", "advertise"])
    }

    /// Withdraw route (retained with empty payload)
    pub fn withdraw(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["routes", "withdraw"])
    }

    /// Subscribe to all route advertisements in mesh
    pub fn all_routes(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["+", "routes", "advertise"])
    }
}

/// Peer-to-peer signaling topics (transient, QoS 0)
pub struct SignalingTopics;

impl SignalingTopics {
    /// ICE candidate exchange
    pub fn ice_candidate(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["signal", "ice", &from.to_string()])
    }

    /// Hole punch coordination
    pub fn hole_punch(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["signal", "holepunch", &from.to_string()])
    }

    /// NAT mapping info
    pub fn nat_info(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["signal", "nat"])
    }

    /// Relay connection request
    pub fn relay_request(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["signal", "relay", &from.to_string()])
    }
}

/// Key management topics
pub struct KeyTopics;

impl KeyTopics {
    /// Node's long-term identity key (retained)
    pub fn identity(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["keys", "identity"])
    }

    /// Ephemeral session key rotation (retained)
    pub fn session(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["keys", "session"])
    }

    /// Key revocation (retained)
    pub fn revocation(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["keys", "revocation"])
    }
}

/// ACL topics (retained)
pub struct AclTopics;

impl AclTopics {
    /// Mesh-wide ACL policy (retained)
    pub fn policy(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["acl", "policy"])
    }

    /// Node-specific ACL overrides (retained)
    pub fn node_override(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["acl", "override"])
    }
}

/// DNS topics
pub struct DnsTopics;

impl DnsTopics {
    /// Mesh DNS records (retained)
    pub fn records(mesh: &MeshName) -> String {
        mesh_topic(mesh, &["dns", "records"])
    }

    /// DNS query (transient)
    pub fn query(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["dns", "query"])
    }

    /// DNS response (transient)
    pub fn response(mesh: &MeshName, node: &NodeId) -> String {
        node_topic(mesh, node, &["dns", "response"])
    }
}

/// File transfer topics
pub struct FileTransferTopics;

impl FileTransferTopics {
    /// File transfer offer (transient)
    pub fn offer(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["file", "offer", &from.to_string()])
    }

    /// File transfer acceptance (transient)
    pub fn accept(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, from, &["file", "accept", &to.to_string()])
    }

    /// File chunk (transient, QoS 1 for reliability)
    pub fn chunk(mesh: &MeshName, from: &NodeId, to: &NodeId, transfer_id: &str) -> String {
        node_topic(mesh, to, &["file", "chunk", &from.to_string(), transfer_id])
    }
}

/// SSH topics
pub struct SshTopics;

impl SshTopics {
    /// SSH session request (transient)
    pub fn request(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["ssh", "request", &from.to_string()])
    }

    /// SSH session data (transient, QoS 1)
    pub fn data(mesh: &MeshName, from: &NodeId, to: &NodeId, session_id: &str) -> String {
        node_topic(mesh, to, &["ssh", "data", &from.to_string(), session_id])
    }
}

/// HTTP proxy topics
pub struct ProxyTopics;

impl ProxyTopics {
    /// Proxy request (transient)
    pub fn request(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String {
        node_topic(mesh, to, &["proxy", "request", &from.to_string()])
    }

    /// Proxy response (transient)
    pub fn response(mesh: &MeshName, from: &NodeId, to: &NodeId, request_id: &str) -> String {
        node_topic(mesh, from, &["proxy", "response", &to.to_string(), request_id])
    }
}

/// Coordinator cluster topics (internal)
pub struct CoordinatorTopics;

impl CoordinatorTopics {
    /// Cluster membership (retained)
    pub fn membership() -> String {
        topic(&["coordinator", "membership"])
    }

    /// Leader election
    pub fn election() -> String {
        topic(&["coordinator", "election"])
    }

    /// State replication
    pub fn replication() -> String {
        topic(&["coordinator", "replication"])
    }
}

/// Payload schemas for each topic type

/// Node presence announcement payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceAnnounce {
    pub node_id: NodeId,
    pub identity_pub: DeviceIdentityPublicKey,
    pub session_pub: SessionPublicKey,
    pub endpoints: Vec<Endpoint>,
    pub capabilities: NodeCapabilities,
    pub overlay_ips: Vec<OverlayIp>,
    pub advertised_routes: Vec<OverlayPrefix>,
    pub timestamp: i64,
    pub version: String,
}

/// Heartbeat payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub node_id: NodeId,
    pub timestamp: i64,
    pub active_peers: u32,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub cpu_usage: f32,
    pub mem_usage: u64,
}

/// LWT payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwtPayload {
    pub node_id: NodeId,
    pub reason: LwtReason,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LwtReason {
    GracefulShutdown,
    ConnectionLost,
    ProtocolError,
    ResourceExhausted,
}

/// Node metadata payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub node_id: NodeId,
    pub display_name: Option<String>,
    pub os: String,
    pub arch: String,
    pub version: String,
    pub tags: Vec<String>,
}

/// Route advertisement payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAdvertisement {
    pub node_id: NodeId,
    pub prefix: OverlayPrefix,
    pub metric: u32,
    pub next_hop: Option<OverlayIp>,
    pub communities: Vec<String>, // For policy routing
    pub timestamp: i64,
}

/// ICE candidate payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCandidate {
    pub candidate: String, // SDP candidate line
    pub mid: String,
    pub ufrag: String,
    pub priority: u32,
}

/// Hole punch payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolePunch {
    pub initiator: NodeId,
    pub responder: NodeId,
    pub ports: Vec<u16>, // Ports to try
    pub timestamp: i64,
    pub nonce: u64, // For synchronization
}

/// NAT info payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatInfo {
    pub node_id: NodeId,
    pub nat_type: NatType,
    pub external_ip: OverlayIp,
    pub external_port: u16,
    pub mapping_behavior: MappingBehavior,
    pub filtering_behavior: FilteringBehavior,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NatType {
    Unknown,
    FullCone,
    RestrictedCone,
    PortRestrictedCone,
    Symmetric,
    Cgnat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MappingBehavior {
    EndpointIndependent,
    AddressDependent,
    AddressAndPortDependent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilteringBehavior {
    EndpointIndependent,
    AddressDependent,
    AddressAndPortDependent,
}

/// Relay request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayRequest {
    pub requester: NodeId,
    pub target: NodeId,
    pub relay_candidates: Vec<Endpoint>,
    pub timestamp: i64,
}

/// Key revocation payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRevocation {
    pub revoked_key: KeyFingerprint,
    pub revoked_by: NodeId,
    pub reason: RevocationReason,
    pub timestamp: i64,
    pub signature: oxide_crypto::keys::DeviceSignature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationReason {
    KeyCompromise,
    KeyRotation,
    NodeDecommissioned,
    PolicyViolation,
}

/// ACL policy payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclPolicy {
    pub version: u64,
    pub default_action: AclAction,
    pub rules: Vec<AclRule>,
    pub timestamp: i64,
    pub signature: oxide_crypto::keys::DeviceSignature,
}

impl Default for AclPolicy {
    fn default() -> Self {
        Self {
            version: 0,
            default_action: AclAction::Allow,
            rules: Vec::new(),
            timestamp: 0,
            signature: oxide_crypto::keys::DeviceSignature::from_bytes(&[0; 64]).unwrap(),
        }
    }
}

/// ACL rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRule {
    pub id: String,
    pub action: AclAction,
    pub src_identities: Vec<KeyFingerprint>,
    pub dst_prefixes: Vec<OverlayPrefix>,
    pub protocols: Vec<u8>,
    pub port_ranges: Vec<(u16, u16)>,
    pub direction: AclDirection,
    pub log: bool,
    pub priority: u32, // Higher = more specific
}



/// DNS record payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: DnsRecordType,
    pub data: String,
    pub ttl: u32,
    pub source: DnsSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsRecordType {
    A,
    AAAA,
    CNAME,
    TXT,
    PTR,
    SRV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsSource {
    Static,
    Dynamic,
    SubnetRouter,
    External,
}