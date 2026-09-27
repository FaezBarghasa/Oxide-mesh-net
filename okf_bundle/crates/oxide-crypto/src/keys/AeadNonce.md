---
okf_version: "0.2"
type: Class
title: AeadNonce
description: AEAD nonce (12 bytes for ChaCha20-Poly1305 / AES-GCM)
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/AeadNonce
language: rust
---

# AeadNonce

AEAD nonce (12 bytes for ChaCha20-Poly1305 / AES-GCM)

## Signature

```rust
pub struct AeadNonce
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

AEAD nonce (12 bytes for ChaCha20-Poly1305 / AES-GCM)
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 356–356 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
| called_by | [random_nonce](/crates/oxide-crypto/src/aead/random_nonce.md) |
| called_by | [derive_aead_key](/crates/oxide-crypto/src/keys/derive_aead_key.md) |
