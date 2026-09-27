---
okf_version: "0.2"
type: Function
title: from_bytes
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/from_bytes_3
language: rust
---

# from_bytes

## Signature

```rust
pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, crate::error::CryptoError>
```

## Visibility

- `pub`

## Source
Lines 89–93 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
