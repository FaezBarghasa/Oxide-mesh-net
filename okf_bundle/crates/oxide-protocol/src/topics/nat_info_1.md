---
okf_version: "0.2"
type: Function
title: nat_info
description: NAT mapping info
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/nat_info_1
language: rust
---

# nat_info

NAT mapping info

## Signature

```rust
pub fn nat_info(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

NAT mapping info

## Source
Lines 126–128 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
