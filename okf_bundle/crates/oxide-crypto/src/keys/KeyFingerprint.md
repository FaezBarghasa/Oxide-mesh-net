---
okf_version: "0.2"
type: Class
title: KeyFingerprint
description: Key fingerprint (Blake3 hash of public key)
resource: crates/oxide-crypto/src/keys.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-crypto"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-crypto/src/keys/KeyFingerprint
language: rust
---

# KeyFingerprint

Key fingerprint (Blake3 hash of public key)

## Signature

```rust
pub struct KeyFingerprint
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Key fingerprint (Blake3 hash of public key)
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 164–164 in `crates/oxide-crypto/src/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-crypto/src/keys.md) |
| called_by | [fingerprint](/crates/oxide-crypto/src/keys/fingerprint.md) |
