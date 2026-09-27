---
okf_version: "0.2"
type: Function
title: from_bytes
description: Parse from bytes
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/from_bytes
language: rust
---

# from_bytes

Parse from bytes

## Signature

```rust
impl WirePacket { pub fn from_bytes(bytes: &[u8]) -> Result<Self> }
```

## Visibility

- `pub`

## Docstring

Parse from bytes

## Source
Lines 98–116 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
