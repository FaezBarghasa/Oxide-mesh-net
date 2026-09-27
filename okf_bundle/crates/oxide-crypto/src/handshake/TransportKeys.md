---
okf_version: "0.2"
type: Class
title: TransportKeys
description: Transport keys derived from handshake
resource: crates/oxide-crypto/src/handshake.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/handshake/TransportKeys
language: rust
---

# TransportKeys

Transport keys derived from handshake

## Signature

```rust
pub struct TransportKeys
```

## Decorators

- `derive(Clone, Zeroize, zeroize::ZeroizeOnDrop)`

## Visibility

- `pub`

## Docstring

Transport keys derived from handshake
[derive(Clone, Zeroize, zeroize::ZeroizeOnDrop)]

## Methods

- `tx_key`
- `rx_key`
- `tx_nonce`
- `rx_nonce`

## Source
Lines 184–189 in `crates/oxide-crypto/src/handshake.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handshake](/crates/oxide-crypto/src/handshake.md) |
