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
timestamp: "2026-09-27T06:16:26Z"
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

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| called_by | [decrypt](/crates/oxide-crypto/src/handshake/decrypt.md) |
| called_by | [encrypt](/crates/oxide-crypto/src/handshake/encrypt.md) |
| called_by | [initiator](/crates/oxide-crypto/src/handshake/initiator.md) |
| called_by | [read_message_1](/crates/oxide-crypto/src/handshake/read_message_1.md) |
