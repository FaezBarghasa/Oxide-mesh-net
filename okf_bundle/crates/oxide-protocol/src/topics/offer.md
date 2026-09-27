---
okf_version: "0.2"
type: Function
title: offer
description: File transfer offer (transient)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/offer
language: rust
---

# offer

File transfer offer (transient)

## Signature

```rust
impl FileTransferTopics { pub fn offer(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

File transfer offer (transient)

## Source
Lines 196–198 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
