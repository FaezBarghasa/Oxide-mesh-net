---
okf_version: "0.2"
type: Function
title: hole_punch
description: Hole punch coordination
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/hole_punch
language: rust
---

# hole_punch

Hole punch coordination

## Signature

```rust
impl SignalingTopics { pub fn hole_punch(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Hole punch coordination

## Source
Lines 121–123 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
