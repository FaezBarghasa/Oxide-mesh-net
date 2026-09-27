---
okf_version: "0.2"
type: Function
title: packet_type
description: Get packet type
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/packet_type_2
language: rust
---

# packet_type

Get packet type

## Signature

```rust
impl WirePacket { pub fn packet_type(&self) -> Result<PacketType> }
```

## Visibility

- `pub`

## Docstring

Get packet type

## Source
Lines 119–121 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
