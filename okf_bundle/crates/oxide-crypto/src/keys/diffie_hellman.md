---
okf_version: "0.2"
type: Function
title: diffie_hellman
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/diffie_hellman
language: rust
---

# diffie_hellman

## Signature

```rust
impl SessionKeyPair { pub fn diffie_hellman(&self, peer_public: &SessionPublicKey) -> SharedSecret }
```

## Visibility

- `pub`

## Source
Lines 227–230 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
