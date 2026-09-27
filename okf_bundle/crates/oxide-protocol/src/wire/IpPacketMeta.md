---
okf_version: "0.2"
type: Class
title: IpPacketMeta
description: Encapsulated IP packet metadata
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/IpPacketMeta
language: rust
---

# IpPacketMeta

Encapsulated IP packet metadata

## Signature

```rust
pub struct IpPacketMeta
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Encapsulated IP packet metadata
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `src_ip`
- `dst_ip`
- `protocol`
- `ttl`
- `dscp`

## Source
Lines 159–165 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
