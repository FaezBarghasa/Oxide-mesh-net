---
okf_version: "0.2"
type: Function
title: mesh_topic
description: Mesh-scoped topic
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/mesh_topic
language: rust
---

# mesh_topic

Mesh-scoped topic

## Signature

```rust
pub fn mesh_topic(mesh: &MeshName, parts: &[&str]) -> String
```

## Visibility

- `pub`

## Docstring

Mesh-scoped topic

## Source
Lines 30–35 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [topic](/crates/oxide-protocol/src/topics/topic.md) |
| called_by | [all_mesh_events](/crates/oxide-protocol/src/topics/all_mesh_events.md) |
| called_by | [all_node_events](/crates/oxide-protocol/src/topics/all_node_events.md) |
| called_by | [all_routes](/crates/oxide-protocol/src/topics/all_routes.md) |
| called_by | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
| called_by | [policy](/crates/oxide-protocol/src/topics/policy.md) |
| called_by | [records](/crates/oxide-protocol/src/topics/records.md) |
| called_by | [revocation](/crates/oxide-protocol/src/topics/revocation.md) |
