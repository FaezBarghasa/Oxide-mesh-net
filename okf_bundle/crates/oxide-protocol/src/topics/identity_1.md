---
okf_version: "0.2"
type: Function
title: identity
description: "Node's long-term identity key (retained)"
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/identity_1
language: rust
---

# identity

Node's long-term identity key (retained)

## Signature

```rust
pub fn identity(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

Node's long-term identity key (retained)

## Source
Lines 141–143 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
