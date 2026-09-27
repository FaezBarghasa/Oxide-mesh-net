---
okf_version: "0.2"
type: Function
title: responder
description: Initialize as responder (NK pattern)
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/responder
language: rust
---

# responder

Initialize as responder (NK pattern)

## Signature

```rust
impl HandshakeState { pub fn responder(our_static: SessionKeyPair) -> Self }
```

## Visibility

- `pub`

## Docstring

Initialize as responder (NK pattern)

## Source
Lines 104–122 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| calls | [mix_hash](/crates/oxide-crypto/src/handshake/mix_hash.md) |
