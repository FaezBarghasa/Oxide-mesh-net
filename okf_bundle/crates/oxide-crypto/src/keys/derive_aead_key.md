---
okf_version: "0.2"
type: Function
title: derive_aead_key
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/derive_aead_key
language: rust
---

# derive_aead_key

## Signature

```rust
impl SharedSecret { pub fn derive_aead_key(&self, context: &[u8]) -> (AeadKey, AeadNonce) }
```

## Visibility

- `pub`

## Source
Lines 312–319 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
| calls | [AeadKey](/crates/oxide-crypto/src/keys/AeadKey.md) |
| calls | [AeadNonce](/crates/oxide-crypto/src/keys/AeadNonce.md) |
