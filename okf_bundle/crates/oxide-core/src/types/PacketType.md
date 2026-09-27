---
okf_version: "0.2"
type: Class
title: PacketType
description: Packet discriminator for wire protocol multiplexing
resource: crates/oxide-core/src/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-core/src/types/PacketType
language: rust
---

# PacketType

Packet discriminator for wire protocol multiplexing

## Signature

```rust
pub enum PacketType
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `repr(u8)`

## Visibility

- `pub`

## Docstring

Packet discriminator for wire protocol multiplexing
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[repr(u8)]

## Source
Lines 330–342 in `crates/oxide-core/src/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-core/src/types.md) |
