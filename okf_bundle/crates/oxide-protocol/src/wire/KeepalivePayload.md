---
okf_version: "0.2"
type: Class
title: KeepalivePayload
description: Control keepalive payload
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/KeepalivePayload
language: rust
---

# KeepalivePayload

Control keepalive payload

## Signature

```rust
pub struct KeepalivePayload
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Control keepalive payload
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `node_id`
- `timestamp`
- `capabilities`
- `endpoints`

## Source
Lines 181–186 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
