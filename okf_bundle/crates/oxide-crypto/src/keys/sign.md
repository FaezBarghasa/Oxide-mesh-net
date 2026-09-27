---
okf_version: "0.2"
type: Function
title: sign
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/sign
language: rust
---

# sign

## Signature

```rust
impl DeviceIdentityKey { pub fn sign(&self, msg: &[u8]) -> DeviceSignature }
```

## Visibility

- `pub`

## Source
Lines 52–55 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
| calls | [DeviceSignature](/crates/oxide-crypto/src/keys/DeviceSignature.md) |
