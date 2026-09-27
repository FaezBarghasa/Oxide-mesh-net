---
okf_version: "0.2"
type: Class
title: NatInfo
description: NAT info payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/NatInfo
language: rust
---

# NatInfo

NAT info payload

## Signature

```rust
pub struct NatInfo
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

NAT info payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `nat_type`
- `external_ip`
- `external_port`
- `mapping_behavior`
- `filtering_behavior`
- `timestamp`

## Source
Lines 352–360 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
