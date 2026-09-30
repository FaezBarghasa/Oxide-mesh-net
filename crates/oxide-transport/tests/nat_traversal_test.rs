//! Integration tests for STUN, UPnP, and Magicsock NAT traversal in oxide-transport

use oxide_core::types::NodeId;
use oxide_transport::nat::{MagicsockEngine, PathType, StunClient};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

#[test]
fn test_stun_packet_roundtrip_and_xor_mapping() {
    let tx_id = [0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let req = StunClient::build_binding_request(&tx_id);
    assert_eq!(req.len(), 20);

    // Build synthetic STUN response for 203.0.113.88:41234
    let target_ip = Ipv4Addr::new(203, 0, 113, 88);
    let target_port: u16 = 41234;

    let xor_port = target_port ^ (0x2112A442 >> 16) as u16;
    let xor_ip = u32::from(target_ip) ^ 0x2112A442;

    let mut resp = Vec::new();
    resp.extend_from_slice(&0x0101u16.to_be_bytes()); // Binding Response
    resp.extend_from_slice(&12u16.to_be_bytes()); // Attribute len
    resp.extend_from_slice(&0x2112A442u32.to_be_bytes()); // Magic Cookie
    resp.extend_from_slice(&tx_id);

    // XOR-MAPPED-ADDRESS
    resp.extend_from_slice(&0x0020u16.to_be_bytes());
    resp.extend_from_slice(&8u16.to_be_bytes());
    resp.push(0x00); // Reserved
    resp.push(0x01); // IPv4
    resp.extend_from_slice(&xor_port.to_be_bytes());
    resp.extend_from_slice(&xor_ip.to_be_bytes());

    let mapped = StunClient::parse_binding_response(&resp, &tx_id).expect("valid stun response");
    assert_eq!(mapped.ip(), target_ip);
    assert_eq!(mapped.port(), target_port);
}

#[test]
fn test_magicsock_multi_path_candidate_migration() {
    let engine = MagicsockEngine::new(vec![]);
    let peer_id = NodeId::new();

    let relay_ep: SocketAddr = "65.108.72.19:443".parse().unwrap();
    let stun_ep: SocketAddr = "203.0.113.88:41234".parse().unwrap();
    let upnp_ep: SocketAddr = "203.0.113.88:51820".parse().unwrap();
    let lan_ep: SocketAddr = "192.168.1.150:51820".parse().unwrap();

    // 1. Initial connection via Relay
    engine.add_peer_candidate(peer_id, relay_ep, PathType::RelayedDerp, Some(Duration::from_millis(60)));
    let p = engine.get_active_path(&peer_id).unwrap();
    assert_eq!(p.path_type, PathType::RelayedDerp);

    // 2. STUN endpoint discovered -> should migrate to STUN P2P
    engine.add_peer_candidate(peer_id, stun_ep, PathType::DirectStunP2P, Some(Duration::from_millis(35)));
    let p = engine.get_active_path(&peer_id).unwrap();
    assert_eq!(p.path_type, PathType::DirectStunP2P);

    // 3. UPnP port forward active -> should migrate to UPnP P2P
    engine.add_peer_candidate(peer_id, upnp_ep, PathType::DirectUpnpP2P, Some(Duration::from_millis(30)));
    let p = engine.get_active_path(&peer_id).unwrap();
    assert_eq!(p.path_type, PathType::DirectUpnpP2P);

    // 4. Same LAN detected -> should migrate to Local LAN (highest priority)
    engine.add_peer_candidate(peer_id, lan_ep, PathType::DirectLocalLan, Some(Duration::from_millis(1)));
    let p = engine.get_active_path(&peer_id).unwrap();
    assert_eq!(p.path_type, PathType::DirectLocalLan);
    assert_eq!(p.addr, lan_ep);
}
