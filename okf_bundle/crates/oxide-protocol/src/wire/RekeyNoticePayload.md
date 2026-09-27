---
okf_version: "0.2"
type: Class
title: RekeyNoticePayload
description: Rekey notice payload
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/RekeyNoticePayload
language: rust
---

# RekeyNoticePayload

Rekey notice payload

## Signature

```rust
pub struct RekeyNoticePayload
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Rekey notice payload
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `node_id`
- `new_session_pub`
- `valid_from`
- `valid_until`

## Source
Lines 200–205 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
