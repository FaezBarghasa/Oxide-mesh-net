---
okf_version: "0.2"
type: Class
title: PathDiscoveryPayload
description: Path discovery probe payload
resource: crates/oxide-protocol/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/wire/PathDiscoveryPayload
language: rust
---

# PathDiscoveryPayload

Path discovery probe payload

## Signature

```rust
pub struct PathDiscoveryPayload
```

## Decorators

- `derive(Debug, Clone, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

Path discovery probe payload
[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Methods

- `probe_id`
- `src_node`
- `dst_node`
- `path_mtu`
- `timestamp`

## Source
Lines 190–196 in `crates/oxide-protocol/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-protocol/src/wire.md) |
