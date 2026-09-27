---
okf_version: "0.2"
type: Function
title: revocation
description: Key revocation (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/revocation
language: rust
---

# revocation

Key revocation (retained)

## Signature

```rust
impl KeyTopics { pub fn revocation(mesh: &MeshName) -> String }
```

## Visibility

- `pub`

## Docstring

Key revocation (retained)

## Source
Lines 151–153 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
