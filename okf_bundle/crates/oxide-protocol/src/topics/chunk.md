---
okf_version: "0.2"
type: Function
title: chunk
description: "File chunk (transient, QoS 1 for reliability)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/chunk
language: rust
---

# chunk

File chunk (transient, QoS 1 for reliability)

## Signature

```rust
impl FileTransferTopics { pub fn chunk(mesh: &MeshName, from: &NodeId, to: &NodeId, transfer_id: &str) -> String }
```

## Visibility

- `pub`

## Docstring

File chunk (transient, QoS 1 for reliability)

## Source
Lines 206–208 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
