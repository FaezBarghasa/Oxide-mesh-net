---
okf_version: "0.2"
type: Function
title: announce
description: Announce node presence (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/announce
language: rust
---

# announce

Announce node presence (retained)

## Signature

```rust
impl PresenceTopics { pub fn announce(mesh: &MeshName, node: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

Announce node presence (retained)

## Source
Lines 71–73 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
