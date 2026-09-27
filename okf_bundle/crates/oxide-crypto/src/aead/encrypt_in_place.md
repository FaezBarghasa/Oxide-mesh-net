---
okf_version: "0.2"
type: Function
title: encrypt_in_place
description: Encrypt in-place using ChaCha20-Poly1305 (for zero-copy operations)
resource: crates/oxide-crypto/src/aead.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/aead/encrypt_in_place
language: rust
---

# encrypt_in_place

Encrypt in-place using ChaCha20-Poly1305 (for zero-copy operations)

## Signature

```rust
pub fn encrypt_in_place(
    key: &AeadKey,
    nonce: &AeadNonce,
    buffer: &mut [u8],
    aad: &[u8],
) -> Result<[u8; 16]>
```

## Visibility

- `pub`

## Docstring

Encrypt in-place using ChaCha20-Poly1305 (for zero-copy operations)

## Source
Lines 43–55 in `crates/oxide-crypto/src/aead.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aead](/crates/oxide-crypto/src/aead.md) |
