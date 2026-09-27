---
okf_version: "0.2"
type: Class
title: SessionKeyPair
description: Ephemeral session key pair (X25519 for key agreement)
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/SessionKeyPair
language: rust
---

# SessionKeyPair

Ephemeral session key pair (X25519 for key agreement)

## Signature

```rust
pub struct SessionKeyPair
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

Ephemeral session key pair (X25519 for key agreement)
[derive(Clone)]

## Methods

- `static_secret`
- `public_key`

## Source
Lines 188–191 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
