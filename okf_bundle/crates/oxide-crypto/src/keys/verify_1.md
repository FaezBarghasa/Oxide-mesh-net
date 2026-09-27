---
okf_version: "0.2"
type: Function
title: verify
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/verify_1
language: rust
---

# verify

## Signature

```rust
pub fn verify(
        &self,
        msg: &[u8],
        sig: &DeviceSignature,
    ) -> Result<(), crate::error::CryptoError>
```

## Visibility

- `pub`

## Source
Lines 95–103 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
