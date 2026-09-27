---
okf_version: "0.2"
type: Function
title: handshake_hash
description: Get handshake hash for channel binding
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/handshake_hash
language: rust
---

# handshake_hash

Get handshake hash for channel binding

## Signature

```rust
impl HandshakeState { pub fn handshake_hash(&self) -> [u8; 32] }
```

## Visibility

- `pub`

## Docstring

Get handshake hash for channel binding

## Source
Lines 177–179 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
