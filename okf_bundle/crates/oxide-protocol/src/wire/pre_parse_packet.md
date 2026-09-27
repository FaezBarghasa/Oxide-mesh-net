---
okf_version: "0.2"
type: Function
title: pre_parse_packet
description: Zero-allocation stateless pre-decapsulation parser
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/pre_parse_packet
language: rust
---

# pre_parse_packet

Zero-allocation stateless pre-decapsulation parser

## Signature

```rust
pub fn pre_parse_packet(buffer: &[u8]) -> PreParseVerdict
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Zero-allocation stateless pre-decapsulation parser

Filters junk frames and extracts framing parameters before AEAD decryption and sequence tracking.
[inline]

## Source
Lines 262–301 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
| called_by | [ws_telemetry_tunnel](/crates/oxide-coordinator/src/handlers/tunnel_ws/ws_telemetry_tunnel.md) |
