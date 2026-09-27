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
