---
okf_version: "0.2"
type: Function
title: withdraw
description: Withdraw route (retained with empty payload)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/withdraw
language: rust
---

# withdraw

Withdraw route (retained with empty payload)

## Signature

```rust
impl RouteTopics { pub fn withdraw(mesh: &MeshName, node: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Withdraw route (retained with empty payload)

## Source
Lines 101–103 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
