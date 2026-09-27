---
okf_version: "0.2"
type: Function
title: mix_hash
description: "Mix data into hash: h = HASH(h || data)"
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/mix_hash
language: rust
---

# mix_hash

Mix data into hash: h = HASH(h || data)

## Signature

```rust
fn mix_hash(h: &mut [u8; 32], data: &[u8])
```

## Docstring

Mix data into hash: h = HASH(h || data)

## Source
Lines 206–211 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| called_by | [initiator](/crates/oxide-crypto/src/handshake/initiator.md) |
| called_by | [read_message_1](/crates/oxide-crypto/src/handshake/read_message_1.md) |
| called_by | [responder](/crates/oxide-crypto/src/handshake/responder.md) |
