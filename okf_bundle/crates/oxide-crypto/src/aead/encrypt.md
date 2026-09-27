---
okf_version: "0.2"
type: Function
title: encrypt
description: Encrypt data using ChaCha20-Poly1305
resource: crates/oxide-crypto/src/aead.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/aead/encrypt
language: rust
---

# encrypt

Encrypt data using ChaCha20-Poly1305

## Signature

```rust
pub fn encrypt(key: &AeadKey, nonce: &AeadNonce, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>>
```

## Visibility

- `pub`

## Docstring

Encrypt data using ChaCha20-Poly1305

## Source
Lines 13–25 in `crates/oxide-crypto/src/aead.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aead](/crates/oxide-crypto/src/aead.md) |
