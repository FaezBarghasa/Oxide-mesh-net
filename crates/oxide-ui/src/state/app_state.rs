//! Central reactive state container for oxide-ui

use crate::models::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Dashboard,
    Routing,
    Obfuscation,
    Identity,
    Services,
    Audit,
    Platform,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub current_view: ActiveView,
    pub node_state: NodeState,
    pub is_reconnecting: bool,
    pub reconnect_attempts: u32,

    // Telemetry & Metrics
    pub telemetry: TelemetrySnapshot,
    pub active_peers: Vec<PeerCardData>,
    pub selected_peer_diagnostics: Option<PeerDeepDiagnostics>,
    pub is_drawer_open: bool,

    // Routing
    pub exit_nodes: Vec<ExitNodeOption>,
    pub advertised_subnets: Vec<AdvertisedSubnet>,
    pub split_rules: Vec<SplitRouteRule>,
    pub route_test_result: Option<RouteDecisionTestResult>,

    // Obfuscation
    pub dpi_config: DpiObfuscationConfig,
    pub reality_config: RealityTlsConfig,
    pub frag_config: ClientHelloFragConfig,
    pub port_hop_state: PortHoppingState,

    // Identity & ACL
    pub device_auth: Option<DeviceAuthSession>,
    pub acl_rules: Vec<AclRule>,
    pub hsm_status: HardwareSecurityStatus,
    pub acl_simulation_result: Option<String>,

    // Services
    pub magic_dns: MagicDnsConfig,
    pub drop_transfers: Vec<OxideDropTransfer>,
    pub active_ssh_sessions: Vec<OxideSshSessionInfo>,
    pub ingress_services: Vec<IngressService>,

    // Audit & Debug
    pub merkle_logs: Vec<MerkleAuditLogEntry>,
    pub packet_stream: Vec<PacketStreamEntry>,
    pub packet_filter_query: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: ActiveView::Dashboard,
            node_state: NodeState::Connected,
            is_reconnecting: false,
            reconnect_attempts: 0,

            telemetry: TelemetrySnapshot {
                ingress_bytes_sec: 14_250_000, // 14.25 MB/s
                egress_bytes_sec: 8_120_000,   // 8.12 MB/s
                pps_in: 9_420,
                pps_out: 5_310,
                current_mtu: 1420,
                overlay_ipv4: "100.64.0.42".to_string(),
                overlay_ipv6: "fd00:0x1d:e::42".to_string(),
                active_threads: 8,
                nat_type: NatType::RestrictedCone,
                throughput_history: vec![
                    4.2, 5.8, 6.1, 8.4, 9.2, 11.5, 13.2, 14.8, 12.4, 14.2, 13.8, 14.25,
                ],
                latency_history: vec![
                    24.0, 23.5, 25.1, 22.8, 21.4, 23.0, 22.2, 21.9, 22.5, 21.8, 22.0, 21.5,
                ],
            },

            active_peers: vec![
                PeerCardData {
                    id: "peer-node-eu-01".to_string(),
                    hostname: "gateway-frankfurt.mesh.oxide".to_string(),
                    overlay_ip: "100.64.0.1".to_string(),
                    os: "Linux (NixOS)".to_string(),
                    connection_vector: PeerConnectionVector::DirectP2P,
                    remote_endpoint: "194.26.29.110:51820".to_string(),
                    rtt_ms: 22.4,
                    last_handshake_secs: 4,
                    bytes_rx: 840_200_000,
                    bytes_tx: 412_500_000,
                    is_online: true,
                    tags: vec!["tag:gateway".to_string(), "tag:prod".to_string()],
                },
                PeerCardData {
                    id: "peer-node-us-02".to_string(),
                    hostname: "dev-server-ashburn.mesh.oxide".to_string(),
                    overlay_ip: "100.64.0.2".to_string(),
                    os: "Linux (Pop!_OS)".to_string(),
                    connection_vector: PeerConnectionVector::DirectP2P,
                    remote_endpoint: "104.244.42.1:51820".to_string(),
                    rtt_ms: 88.6,
                    last_handshake_secs: 12,
                    bytes_rx: 124_000_000,
                    bytes_tx: 98_000_000,
                    is_online: true,
                    tags: vec!["tag:dev".to_string()],
                },
                PeerCardData {
                    id: "peer-node-relay-03".to_string(),
                    hostname: "masque-relay-helsinki.mesh.oxide".to_string(),
                    overlay_ip: "100.64.0.99".to_string(),
                    os: "Linux (Debian)".to_string(),
                    connection_vector: PeerConnectionVector::MasqueRelayed,
                    remote_endpoint: "65.108.72.19:443".to_string(),
                    rtt_ms: 48.2,
                    last_handshake_secs: 1,
                    bytes_rx: 54_000_000,
                    bytes_tx: 32_000_000,
                    is_online: true,
                    tags: vec!["tag:relay".to_string()],
                },
            ],

            selected_peer_diagnostics: None,
            is_drawer_open: false,

            exit_nodes: vec![
                ExitNodeOption {
                    id: "peer-node-eu-01".to_string(),
                    hostname: "Frankfurt Egress 01".to_string(),
                    location_country: "Germany".to_string(),
                    location_city: "Frankfurt am Main".to_string(),
                    latency_ms: 22.4,
                    is_active: true,
                    allows_lan_access: true,
                    verified_public_ip: Some("194.26.29.110".to_string()),
                    dns_leak_protected: true,
                },
                ExitNodeOption {
                    id: "peer-node-us-02".to_string(),
                    hostname: "Ashburn Egress 02".to_string(),
                    location_country: "United States".to_string(),
                    location_city: "Ashburn, VA".to_string(),
                    latency_ms: 88.6,
                    is_active: false,
                    allows_lan_access: true,
                    verified_public_ip: None,
                    dns_leak_protected: true,
                },
            ],

            advertised_subnets: vec![AdvertisedSubnet {
                cidr: "192.168.10.0/24".to_string(),
                interface_name: "eth0.lan".to_string(),
                is_active: true,
                has_conflict: false,
                active_vrrp_leader: Some("local-node".to_string()),
            }],

            split_rules: vec![
                SplitRouteRule {
                    id: "rule-1".to_string(),
                    pattern: "*.local, 192.168.1.0/24".to_string(),
                    bypass_mesh: true,
                    target_app_process: None,
                },
                SplitRouteRule {
                    id: "rule-2".to_string(),
                    pattern: "apt.pop-os.org".to_string(),
                    bypass_mesh: true,
                    target_app_process: Some("apt".to_string()),
                },
            ],

            route_test_result: None,

            dpi_config: DpiObfuscationConfig {
                magic_header_scramble_enabled: true,
                custom_magic_hex: "0x8F4E2A1B".to_string(),
                padding_min_bytes: 40,
                padding_max_bytes: 280,
                pre_handshake_junk_enabled: true,
                junk_burst_count: 4,
                junk_payload_max_bytes: 512,
            },

            reality_config: RealityTlsConfig {
                enabled: true,
                sni_target: "www.microsoft.com".to_string(),
                server_public_key_hex:
                    "3a9f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a".to_string(),
                short_id_hex: "0123456789abcdef".to_string(),
                diverted_probes_count: 142,
                remote_sni_reachable: true,
            },

            frag_config: ClientHelloFragConfig {
                enabled: true,
                split_byte_offset: 24,
                split_delay_ms: 4,
                is_actively_splitting: true,
            },

            port_hop_state: PortHoppingState {
                enabled: true,
                port_range_start: 20000,
                port_range_end: 60000,
                hop_interval_secs: 30,
                current_port: 34182,
                next_port_candidate: 51204,
                seconds_to_next_hop: 18,
                emergency_wss_fallback_active: false,
                enforce_wss_only: false,
            },

            device_auth: Some(DeviceAuthSession {
                verification_uri: "https://auth.oxide.mesh/device".to_string(),
                user_code: "OXID-7842".to_string(),
                expires_in_secs: 480,
                is_approved: false,
            }),

            acl_rules: vec![
                AclRule {
                    id: "acl-1".to_string(),
                    src_tag: "tag:dev".to_string(),
                    dst_tag: "tag:gateway".to_string(),
                    proto_mask: 0x06, // TCP
                    port_range_start: 443,
                    port_range_end: 443,
                    action_allow: true,
                },
                AclRule {
                    id: "acl-2".to_string(),
                    src_tag: "tag:dev".to_string(),
                    dst_tag: "tag:prod".to_string(),
                    proto_mask: 0xFF,
                    port_range_start: 0,
                    port_range_end: 65535,
                    action_allow: false,
                },
            ],

            hsm_status: HardwareSecurityStatus {
                hsm_type: "TPM 2.0 (Endorsement Key Auth)".to_string(),
                tpm2_bound: true,
                cert_algo: "Ed25519 + ML-KEM-768 (Kyber Hybrid)".to_string(),
                cert_serial: "0x7F9A-4B2C-1E8D".to_string(),
                expires_at: "2027-09-26 18:00:00 UTC".to_string(),
                coordinator_signed: true,
            },

            acl_simulation_result: None,

            magic_dns: MagicDnsConfig {
                fqdn: "workstation-popos.mesh.oxide".to_string(),
                aliases: vec!["ws.oxide".to_string(), "local.oxide".to_string()],
                search_domains: vec!["mesh.oxide".to_string(), "corp.internal".to_string()],
                doh_upstream: "1.1.1.1 (Cloudflare DoH Direct)".to_string(),
                is_resolution_active: true,
            },

            drop_transfers: vec![OxideDropTransfer {
                transfer_id: "tx-492".to_string(),
                filename: "kernel-image-v6.11.tar.zst".to_string(),
                file_size_bytes: 42_500_000,
                peer_id: "peer-node-eu-01".to_string(),
                peer_hostname: "gateway-frankfurt.mesh.oxide".to_string(),
                is_incoming: false,
                progress_ratio: 0.76,
                speed_mbps: 48.2,
                blake3_verified: true,
                is_completed: false,
            }],

            active_ssh_sessions: vec![OxideSshSessionInfo {
                session_id: "ssh-session-01".to_string(),
                target_peer: "gateway-frankfurt.mesh.oxide".to_string(),
                cipher_suite: "ChaCha20-Poly1305 / Ed25519".to_string(),
                active_duration_secs: 840,
                is_recording: false,
            }],

            ingress_services: vec![IngressService {
                service_id: "srv-1".to_string(),
                local_bind_addr: "127.0.0.1:8080".to_string(),
                magic_dns_alias: "http://dev-api.mesh.oxide".to_string(),
                is_public_funnel: true,
                public_url: Some("https://dev-api.funnel.oxide.net".to_string()),
                acme_tls_provisioned: true,
                requests_handled: 18_420,
            }],

            merkle_logs: vec![
                MerkleAuditLogEntry {
                    sequence_num: 1042,
                    timestamp: "2026-09-26 19:42:11".to_string(),
                    action_type: "ACL_POLICY_MUTATION".to_string(),
                    initiator_node_id: "node-admin-01".to_string(),
                    signature_hex: "0x4a9f...e18b".to_string(),
                    merkle_leaf_hash: "0x8b2c...7f1a".to_string(),
                    root_hash_verified: true,
                },
                MerkleAuditLogEntry {
                    sequence_num: 1041,
                    timestamp: "2026-09-26 19:30:05".to_string(),
                    action_type: "EXIT_NODE_ELECTED".to_string(),
                    initiator_node_id: "node-local-self".to_string(),
                    signature_hex: "0x91d3...2c4a".to_string(),
                    merkle_leaf_hash: "0x3e4f...110a".to_string(),
                    root_hash_verified: true,
                },
            ],

            packet_stream: vec![
                PacketStreamEntry {
                    timestamp_us: 1727376000000,
                    src_ip: "100.64.0.42".to_string(),
                    dst_ip: "100.64.0.1".to_string(),
                    protocol: "UDP/QUIC".to_string(),
                    src_port: 34182,
                    dst_port: 51820,
                    payload_bytes: 1420,
                    is_dropped: false,
                    drop_reason: None,
                },
                PacketStreamEntry {
                    timestamp_us: 1727376000045,
                    src_ip: "100.64.0.42".to_string(),
                    dst_ip: "100.64.0.2".to_string(),
                    protocol: "TCP".to_string(),
                    src_port: 52140,
                    dst_port: 22,
                    payload_bytes: 64,
                    is_dropped: true,
                    drop_reason: Some(
                        "ACL Drop: rule #acl-2 (tag:dev -> tag:prod DENIED)".to_string(),
                    ),
                },
            ],

            packet_filter_query: String::new(),
        }
    }
}
