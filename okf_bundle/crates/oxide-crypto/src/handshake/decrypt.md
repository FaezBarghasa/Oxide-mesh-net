---
okf_version: "0.2"
type: Function
title: decrypt
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/decrypt
language: rust
---

# decrypt

## Signature

```rust
impl TransportKeys { pub fn decrypt(&mut self, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> }
```

## Visibility

- `pub`

## Source
Lines 198–202 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [nonce_bytes](/crates/oxide-crypto/src/handshake/nonce_bytes.md) |
