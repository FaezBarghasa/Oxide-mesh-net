---
okf_version: "0.2"
type: Class
title: RouteAdvertisement
description: Route advertisement payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/RouteAdvertisement
language: rust
---

# RouteAdvertisement

Route advertisement payload

## Signature

```rust
pub struct RouteAdvertisement
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Route advertisement payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `prefix`
- `metric`
- `next_hop`
- `communities`
- `timestamp`

## Source
Lines 322–329 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
