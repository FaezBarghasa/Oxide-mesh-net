---
okf_version: "0.2"
type: Function
title: relay_request
description: Relay connection request
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/relay_request
language: rust
---

# relay_request

Relay connection request

## Signature

```rust
impl SignalingTopics { pub fn relay_request(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Relay connection request

## Source
Lines 131–133 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
