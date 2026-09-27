//! Continuous Fuzzing & Adversarial Wire Frame Parser Tests
//!
//! Validates that malformed envelopes, truncated frames, random junk, and invalid sequence
//! mutations never panic or compromise cryptographic state.

use oxide_core::PacketType;
use oxide_protocol::wire::{PreParseVerdict, ReplayWindow128, WirePacket, pre_parse_packet};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_pre_parse_arbitrary_bytes_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
        let verdict = pre_parse_packet(&bytes);
        match verdict {
            PreParseVerdict::ValidData { packet_id: _, payload_len } |
            PreParseVerdict::ValidControl { packet_id: _, payload_len } => {
                assert!(bytes.len() >= 16 + payload_len as usize);
            }
            PreParseVerdict::JunkIgnored | PreParseVerdict::Malformed => {}
        }
    }

    #[test]
    fn test_replay_window_arbitrary_sequences_invariants(seqs in proptest::collection::vec(1u64..100000, 1..500)) {
        let mut window = ReplayWindow128::new();
        for seq in seqs {
            let res = window.check_and_update(seq);
            if res {
                // If sequence was accepted once, immediate resubmission must be rejected (replay protection)
                assert!(!window.check_and_update(seq));
            }
        }
    }

    #[test]
    fn test_wire_packet_from_bytes_fuzz_safety(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let _ = WirePacket::from_bytes(&bytes);
    }

    #[test]
    fn test_wire_packet_roundtrip_fuzz(
        packet_id in any::<u32>(),
        payload in proptest::collection::vec(any::<u8>(), 0..1024)
    ) {
        let packet = WirePacket::new(PacketType::Ipv4, packet_id, payload.clone()).unwrap();
        let bytes = packet.to_bytes();
        let parsed = WirePacket::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.header.packet_id, packet_id);
        assert_eq!(parsed.payload, payload);
    }
}
