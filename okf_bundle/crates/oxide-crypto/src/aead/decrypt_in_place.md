---
okf_version: "0.2"
type: Function
title: decrypt_in_place
description: Decrypt in-place using ChaCha20-Poly1305
resource: crates/oxide-crypto/src/aead.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/aead/decrypt_in_place
language: rust
---

# decrypt_in_place

Decrypt in-place using ChaCha20-Poly1305

## Signature

```rust
pub fn decrypt_in_place(
    key: &AeadKey,
    nonce: &AeadNonce,
    buffer: &mut [u8],
    tag: &[u8; 16],
    aad: &[u8],
) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Decrypt in-place using ChaCha20-Poly1305

## Source
Lines 58–71 in `crates/oxide-crypto/src/aead.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aead](/crates/oxide-crypto/src/aead.md) |
