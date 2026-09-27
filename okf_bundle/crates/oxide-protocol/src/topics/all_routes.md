---
okf_version: "0.2"
type: Function
title: all_routes
description: Subscribe to all route advertisements in mesh
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/all_routes
language: rust
---

# all_routes

Subscribe to all route advertisements in mesh

## Signature

```rust
impl RouteTopics { pub fn all_routes(mesh: &MeshName) -> String }
```

## Visibility

- `pub`

## Docstring

Subscribe to all route advertisements in mesh

## Source
Lines 106–108 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
