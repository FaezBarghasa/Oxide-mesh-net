---
okf_version: "0.2"
type: Function
title: records
description: Mesh DNS records (retained)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/records
language: rust
---

# records

Mesh DNS records (retained)

## Signature

```rust
impl DnsTopics { pub fn records(mesh: &MeshName) -> String }
```

## Visibility

- `pub`

## Docstring

Mesh DNS records (retained)

## Source
Lines 176–178 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
