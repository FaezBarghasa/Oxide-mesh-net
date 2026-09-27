---
okf_version: "0.2"
type: Function
title: to_bytes
description: Serialize to bytes (zero-copy when possible)
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/to_bytes_1
language: rust
---

# to_bytes

Serialize to bytes (zero-copy when possible)

## Signature

```rust
pub fn to_bytes(&self) -> Vec<u8>
```

## Visibility

- `pub`

## Docstring

Serialize to bytes (zero-copy when possible)

## Source
Lines 90–95 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
