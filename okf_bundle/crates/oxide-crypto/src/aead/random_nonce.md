---
okf_version: "0.2"
type: Function
title: random_nonce
description: Generate a random nonce
resource: crates/oxide-crypto/src/aead.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/aead/random_nonce
language: rust
---

# random_nonce

Generate a random nonce

## Signature

```rust
pub fn random_nonce() -> AeadNonce
```

## Visibility

- `pub`

## Docstring

Generate a random nonce

## Source
Lines 74–81 in `crates/oxide-crypto/src/aead.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [aead](/crates/oxide-crypto/src/aead.md) |
| calls | [AeadNonce](/crates/oxide-crypto/src/keys/AeadNonce.md) |
