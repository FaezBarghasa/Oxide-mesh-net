---
okf_version: "0.2"
type: Function
title: data
description: "SSH session data (transient, QoS 1)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/data_1
language: rust
---

# data

SSH session data (transient, QoS 1)

## Signature

```rust
pub fn data(mesh: &MeshName, from: &NodeId, to: &NodeId, session_id: &str) -> String
```

## Visibility

- `pub`

## Docstring

SSH session data (transient, QoS 1)

## Source
Lines 221–223 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
