---
okf_version: "0.2"
type: Function
title: accept
description: File transfer acceptance (transient)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/accept_1
language: rust
---

# accept

File transfer acceptance (transient)

## Signature

```rust
pub fn accept(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

File transfer acceptance (transient)

## Source
Lines 201–203 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
