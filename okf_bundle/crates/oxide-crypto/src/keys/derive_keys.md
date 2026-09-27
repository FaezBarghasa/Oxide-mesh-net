---
okf_version: "0.2"
type: Function
title: derive_keys
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/derive_keys
language: rust
---

# derive_keys

## Signature

```rust
impl SharedSecret { pub fn derive_keys(&self, salt: &[u8], info: &[u8], output_len: usize) -> Vec<u8> }
```

## Visibility

- `pub`

## Source
Lines 304–310 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
