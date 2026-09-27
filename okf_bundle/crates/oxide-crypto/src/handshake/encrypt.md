---
okf_version: "0.2"
type: Function
title: encrypt
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/encrypt
language: rust
---

# encrypt

## Signature

```rust
impl TransportKeys { pub fn encrypt(&mut self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> }
```

## Visibility

- `pub`

## Source
Lines 192–196 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [nonce_bytes](/crates/oxide-crypto/src/handshake/nonce_bytes.md) |
