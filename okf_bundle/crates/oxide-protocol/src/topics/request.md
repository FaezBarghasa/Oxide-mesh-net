---
okf_version: "0.2"
type: Function
title: request
description: SSH session request (transient)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/request
language: rust
---

# request

SSH session request (transient)

## Signature

```rust
impl SshTopics { pub fn request(mesh: &MeshName, from: &NodeId, to: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

SSH session request (transient)

## Source
Lines 216–218 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
