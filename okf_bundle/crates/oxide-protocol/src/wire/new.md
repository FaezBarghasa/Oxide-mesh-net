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
concept_id: crates/oxide-protocol/src/wire/new
language: rust
---

# new

## Signature

```rust
impl PacketHeader { pub fn new(packet_type: PacketType, packet_id: u32, payload_len: u16) -> Self }
```

## Visibility

- `pub`

## Source
Lines 31–41 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
