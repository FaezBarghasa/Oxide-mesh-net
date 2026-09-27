---
okf_version: "0.2"
type: Function
title: session
description: Ephemeral session key rotation (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/session_1
language: rust
---

# session

Ephemeral session key rotation (retained)

## Signature

```rust
pub fn session(mesh: &MeshName, node: &NodeId) -> String
```

## Visibility

- `pub`

## Docstring

Ephemeral session key rotation (retained)

## Source
Lines 146–148 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
