---
okf_version: "0.2"
type: Function
title: lwt
description: "Last Will Testament (set on connect, published on ungraceful disconnect)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/lwt_1
language: rust
---

# lwt

Last Will Testament (set on connect, published on ungraceful disconnect)

## Signature

```rust
pub fn lwt(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

Last Will Testament (set on connect, published on ungraceful disconnect)

## Source
Lines 81–83 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
