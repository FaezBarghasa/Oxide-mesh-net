---
okf_version: "0.2"
type: Class
title: KeyRevocation
description: Key revocation payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/KeyRevocation
language: rust
---

# KeyRevocation

Key revocation payload

## Signature

```rust
pub struct KeyRevocation
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Key revocation payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `revoked_key`
- `revoked_by`
- `reason`
- `timestamp`
- `signature`

## Source
Lines 397–403 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
