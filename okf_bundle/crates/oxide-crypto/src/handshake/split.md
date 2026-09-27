---
okf_version: "0.2"
type: Function
title: split
description: Complete handshake and derive transport keys
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/split
language: rust
---

# split

Complete handshake and derive transport keys

## Signature

```rust
impl HandshakeState { pub fn split(self) -> Result<TransportKeys> }
```

## Visibility

- `pub`

## Docstring

Complete handshake and derive transport keys

## Source
Lines 161–174 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [hkdf2](/crates/oxide-crypto/src/handshake/hkdf2.md) |
