---
okf_version: "0.2"
type: Class
title: NodeInfo
description: "[derive(Serialize)]"
resource: crates/oxide-coordinator/src/handlers/nodes.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/nodes/NodeInfo
language: rust
---

# NodeInfo

[derive(Serialize)]

## Signature

```rust
pub struct NodeInfo
```

## Decorators

- `derive(Serialize)`

## Visibility

- `pub`

## Docstring

[derive(Serialize)]

## Methods

- `node_id`
- `display_name`
- `os`
- `arch`
- `version`
- `last_seen`
- `rx_bytes`
- `tx_bytes`
- `tags`

## Source
Lines 17–27 in `crates/oxide-coordinator/src/handlers/nodes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nodes](/crates/oxide-coordinator/src/handlers/nodes.md) |
