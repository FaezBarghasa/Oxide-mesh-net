---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/new_2
language: rust
---

# new

## Signature

```rust
impl WirePacket { pub fn new(packet_type: PacketType, packet_id: u32, payload: Vec<u8>) -> Result<Self> }
```

## Visibility

- `pub`

## Source
Lines 75–83 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
