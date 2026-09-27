---
okf_version: "0.2"
type: Function
title: node_events
description: Specific node events
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/node_events
language: rust
---

# node_events

Specific node events

## Signature

```rust
impl TopicPatterns { pub fn node_events(mesh: &MeshName, node: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Specific node events

## Source
Lines 61–63 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
