---
okf_version: "0.2"
type: Function
title: node_override
description: Node-specific ACL overrides (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/node_override_1
language: rust
---

# node_override

Node-specific ACL overrides (retained)

## Signature

```rust
pub fn node_override(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

Node-specific ACL overrides (retained)

## Source
Lines 166–168 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
