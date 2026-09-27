---
okf_version: "0.2"
type: Class
title: AeadKey
description: AEAD encryption key (ChaCha20-Poly1305 or AES-GCM)
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/AeadKey
language: rust
---

# AeadKey

AEAD encryption key (ChaCha20-Poly1305 or AES-GCM)

## Signature

```rust
pub struct AeadKey
```

## Decorators

- `derive(Clone, Zeroize, ZeroizeOnDrop)`

## Visibility

- `pub`

## Docstring

AEAD encryption key (ChaCha20-Poly1305 or AES-GCM)
[derive(Clone, Zeroize, ZeroizeOnDrop)]

## Source
Lines 332–332 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
| called_by | [derive_aead_key](/crates/oxide-crypto/src/keys/derive_aead_key.md) |
