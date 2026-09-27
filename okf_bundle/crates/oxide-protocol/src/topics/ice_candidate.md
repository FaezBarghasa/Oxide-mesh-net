---
okf_version: "0.2"
type: Function
title: ice_candidate
description: ICE candidate exchange
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/ice_candidate
language: rust
---

# ice_candidate

ICE candidate exchange

## Signature

```rust
impl SignalingTopics { pub fn ice_candidate(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

ICE candidate exchange

## Source
Lines 116–118 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
