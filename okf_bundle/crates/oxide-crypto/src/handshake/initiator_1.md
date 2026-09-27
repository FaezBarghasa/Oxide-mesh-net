---
okf_version: "0.2"
type: Function
title: initiator
description: Initialize as initiator (NK pattern)
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/initiator_1
language: rust
---

# initiator

Initialize as initiator (NK pattern)

## Signature

```rust
pub fn initiator(our_static: SessionKeyPair, peer_static_pub: SessionPublicKey) -> Self
```

## Visibility

- `pub`

## Docstring

Initialize as initiator (NK pattern)

## Source
Lines 56–101 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [mix_hash](/crates/oxide-crypto/src/handshake/mix_hash.md) |
| calls | [hkdf2](/crates/oxide-crypto/src/handshake/hkdf2.md) |
| calls | [nonce_bytes](/crates/oxide-crypto/src/handshake/nonce_bytes.md) |
