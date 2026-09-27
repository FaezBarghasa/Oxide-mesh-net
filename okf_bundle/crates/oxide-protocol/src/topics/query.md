---
okf_version: "0.2"
type: Function
title: query
description: DNS query (transient)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/query
language: rust
---

# query

DNS query (transient)

## Signature

```rust
impl DnsTopics { pub fn query(mesh: &MeshName, node: &NodeId) -> String }
```

## Visibility

- `pub`

## Docstring

DNS query (transient)

## Source
Lines 181–183 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
