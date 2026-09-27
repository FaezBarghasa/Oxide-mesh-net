---
okf_version: "0.2"
type: Function
title: nonce_bytes
description: Convert u64 to 12-byte nonce (little-endian + zeros)
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
concept_id: crates/oxide-crypto/src/handshake/nonce_bytes
language: rust
---

# nonce_bytes

Convert u64 to 12-byte nonce (little-endian + zeros)

## Signature

```rust
fn nonce_bytes(n: u64) -> [u8; 12]
```

## Docstring

Convert u64 to 12-byte nonce (little-endian + zeros)

## Source
Lines 226–230 in `crates/oxide-crypto/src/handshake.rs`
