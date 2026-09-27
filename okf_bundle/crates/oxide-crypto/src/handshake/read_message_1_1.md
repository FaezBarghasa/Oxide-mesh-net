---
okf_version: "0.2"
type: Function
title: read_message_1
description: "Process initiator's first message (responder side)"
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/read_message_1_1
language: rust
---

# read_message_1

Process initiator's first message (responder side)

## Signature

```rust
pub fn read_message_1(&mut self, msg: &[u8]) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Process initiator's first message (responder side)

## Source
Lines 125–158 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [mix_hash](/crates/oxide-crypto/src/handshake/mix_hash.md) |
| calls | [hkdf2](/crates/oxide-crypto/src/handshake/hkdf2.md) |
| calls | [nonce_bytes](/crates/oxide-crypto/src/handshake/nonce_bytes.md) |
