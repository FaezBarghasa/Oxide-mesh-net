---
okf_version: "0.2"
type: Class
title: HybridSharedSecret
description: "Hybrid shared secret: X25519 || ML-KEM"
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/HybridSharedSecret
language: rust
---

# HybridSharedSecret

Hybrid shared secret: X25519 || ML-KEM

## Signature

```rust
pub struct HybridSharedSecret
```

## Decorators

- `derive(Clone, Zeroize, ZeroizeOnDrop)`

## Visibility

- `pub`

## Docstring

Hybrid shared secret: X25519 || ML-KEM
[derive(Clone, Zeroize, ZeroizeOnDrop)]

## Methods

- `classical`
- `post_quantum`

## Source
Lines 239–242 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
