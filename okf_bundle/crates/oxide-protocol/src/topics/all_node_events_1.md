---
okf_version: "0.2"
type: Function
title: all_node_events
description: All node events in mesh
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/all_node_events_1
language: rust
---

# all_node_events

All node events in mesh

## Signature

```rust
pub fn all_node_events(mesh: &MeshName) -> String
```

## Visibility

- `pub`

## Docstring

All node events in mesh

## Source
Lines 56–58 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
