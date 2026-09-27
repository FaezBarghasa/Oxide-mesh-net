---
okf_version: "0.2"
type: Function
title: policy
description: Mesh-wide ACL policy (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/policy
language: rust
---

# policy

Mesh-wide ACL policy (retained)

## Signature

```rust
impl AclTopics { pub fn policy(mesh: &MeshName) -> String }
```

## Visibility

- `pub`

## Docstring

Mesh-wide ACL policy (retained)

## Source
Lines 161–163 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
