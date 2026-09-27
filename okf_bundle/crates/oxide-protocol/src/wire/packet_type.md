---
okf_version: "0.2"
type: Function
title: packet_type
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/packet_type
language: rust
---

# packet_type

## Signature

```rust
impl PacketHeader { pub fn packet_type(&self) -> Result<PacketType> }
```

## Visibility

- `pub`

## Source
Lines 61–64 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
