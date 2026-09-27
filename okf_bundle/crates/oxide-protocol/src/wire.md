---
okf_version: "0.2"
type: Module
title: wire
description: Wire protocol framing for data plane packets
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire
language: rust
---

# wire

Wire protocol framing for data plane packets

## Docstring

Wire protocol framing for data plane packets

## Relationships

| Type | Target |
|------|--------|
| related | [PacketHeader](/crates/oxide-protocol/src/wire/PacketHeader.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [validate](/crates/oxide-protocol/src/wire/validate.md) |
| related | [packet_type](/crates/oxide-protocol/src/wire/packet_type.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [validate](/crates/oxide-protocol/src/wire/validate.md) |
| related | [packet_type](/crates/oxide-protocol/src/wire/packet_type.md) |
| related | [WirePacket](/crates/oxide-protocol/src/wire/WirePacket.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [total_len](/crates/oxide-protocol/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-protocol/src/wire/to_bytes.md) |
| related | [from_bytes](/crates/oxide-protocol/src/wire/from_bytes.md) |
| related | [packet_type](/crates/oxide-protocol/src/wire/packet_type.md) |
| related | [is_control](/crates/oxide-protocol/src/wire/is_control.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [total_len](/crates/oxide-protocol/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-protocol/src/wire/to_bytes.md) |
| related | [from_bytes](/crates/oxide-protocol/src/wire/from_bytes.md) |
| related | [packet_type](/crates/oxide-protocol/src/wire/packet_type.md) |
| related | [is_control](/crates/oxide-protocol/src/wire/is_control.md) |
| related | [BatchPacket](/crates/oxide-protocol/src/wire/BatchPacket.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [total_len](/crates/oxide-protocol/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-protocol/src/wire/to_bytes.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [total_len](/crates/oxide-protocol/src/wire/total_len.md) |
| related | [to_bytes](/crates/oxide-protocol/src/wire/to_bytes.md) |
| related | [IpPacketMeta](/crates/oxide-protocol/src/wire/IpPacketMeta.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [KeepalivePayload](/crates/oxide-protocol/src/wire/KeepalivePayload.md) |
| related | [PathDiscoveryPayload](/crates/oxide-protocol/src/wire/PathDiscoveryPayload.md) |
| related | [RekeyNoticePayload](/crates/oxide-protocol/src/wire/RekeyNoticePayload.md) |
| related | [AclUpdatePayload](/crates/oxide-protocol/src/wire/AclUpdatePayload.md) |
| related | [AclRuleWire](/crates/oxide-protocol/src/wire/AclRuleWire.md) |
| related | [AclAction](/crates/oxide-protocol/src/wire/AclAction.md) |
| related | [AclDirection](/crates/oxide-protocol/src/wire/AclDirection.md) |
| related | [PreParseVerdict](/crates/oxide-protocol/src/wire/PreParseVerdict.md) |
| related | [pre_parse_packet](/crates/oxide-protocol/src/wire/pre_parse_packet.md) |
| related | [ReplayWindow128](/crates/oxide-protocol/src/wire/ReplayWindow128.md) |
| related | [default](/crates/oxide-protocol/src/wire/default.md) |
| related | [default](/crates/oxide-protocol/src/wire/default.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [check_and_update](/crates/oxide-protocol/src/wire/check_and_update.md) |
| related | [last_sequence](/crates/oxide-protocol/src/wire/last_sequence.md) |
| related | [new](/crates/oxide-protocol/src/wire/new.md) |
| related | [check_and_update](/crates/oxide-protocol/src/wire/check_and_update.md) |
| related | [last_sequence](/crates/oxide-protocol/src/wire/last_sequence.md) |
| related | [test_packet_header_roundtrip](/crates/oxide-protocol/src/wire/test_packet_header_roundtrip.md) |
| related | [test_wire_packet_roundtrip](/crates/oxide-protocol/src/wire/test_wire_packet_roundtrip.md) |
| related | [test_replay_window_in_order](/crates/oxide-protocol/src/wire/test_replay_window_in_order.md) |
| related | [test_replay_window_duplicate_rejection](/crates/oxide-protocol/src/wire/test_replay_window_duplicate_rejection.md) |
| related | [test_replay_window_out_of_order](/crates/oxide-protocol/src/wire/test_replay_window_out_of_order.md) |
| related | [test_replay_window_stale_rejection](/crates/oxide-protocol/src/wire/test_replay_window_stale_rejection.md) |
| related | [test_pre_parse_junk_and_authentic](/crates/oxide-protocol/src/wire/test_pre_parse_junk_and_authentic.md) |
| related | [zerocopy](/_dependencies/cargo/zerocopy.md) |
