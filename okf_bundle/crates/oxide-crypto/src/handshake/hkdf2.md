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
