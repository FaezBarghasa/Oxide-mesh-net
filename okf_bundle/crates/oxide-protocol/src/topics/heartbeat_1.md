---
okf_version: "0.2"
type: Function
title: heartbeat
description: "Node heartbeat (not retained, high frequency)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/heartbeat_1
language: rust
---

# heartbeat

Node heartbeat (not retained, high frequency)

## Signature

```rust
pub fn heartbeat(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

Node heartbeat (not retained, high frequency)

## Source
Lines 76–78 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
