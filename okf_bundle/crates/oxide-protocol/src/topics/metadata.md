---
okf_version: "0.2"
type: Function
title: metadata
description: "Node metadata (retained, updated on config change)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/metadata
language: rust
---

# metadata

Node metadata (retained, updated on config change)

## Signature

```rust
impl PresenceTopics { pub fn metadata(mesh: &MeshName, node: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Node metadata (retained, updated on config change)

## Source
Lines 86–88 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
