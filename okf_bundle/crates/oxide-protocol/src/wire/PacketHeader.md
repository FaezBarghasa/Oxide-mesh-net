---
okf_version: "0.2"
type: Class
title: PacketHeader
description: "Wire packet header (16 bytes, aligned for SIMD)"
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/PacketHeader
language: rust
---

# PacketHeader

Wire packet header (16 bytes, aligned for SIMD)

## Signature

```rust
pub struct PacketHeader
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)`

## Visibility

- `pub`

## Docstring

Wire packet header (16 bytes, aligned for SIMD)
[repr(C)]
[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)]

## Methods

- `magic`
- `version`
- `packet_type`
- `flags`
- `packet_id`
- `payload_len`
- `reserved`

## Source
Lines 18–26 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
