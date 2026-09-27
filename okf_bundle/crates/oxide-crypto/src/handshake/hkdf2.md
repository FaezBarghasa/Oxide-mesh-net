---
okf_version: "0.2"
type: Function
title: hkdf2
description: "HKDF with 2 outputs: (ck', k) = HKDF(ck, input)"
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/hkdf2
language: rust
---

# hkdf2

HKDF with 2 outputs: (ck', k) = HKDF(ck, input)

## Signature

```rust
fn hkdf2(ck: &[u8; 32], input: &[u8]) -> ([u8; 32], [u8; 32])
```

## Docstring

HKDF with 2 outputs: (ck', k) = HKDF(ck, input)

## Source
Lines 214–223 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
| called_by | [initiator](/crates/oxide-crypto/src/handshake/initiator.md) |
| called_by | [read_message_1](/crates/oxide-crypto/src/handshake/read_message_1.md) |
| called_by | [split](/crates/oxide-crypto/src/handshake/split.md) |
