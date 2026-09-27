---
okf_version: "0.2"
type: Function
title: response
description: Proxy response (transient)
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/response_2
language: rust
---

# response

Proxy response (transient)

## Signature

```rust
impl ProxyTopics { pub fn response(mesh: &MeshName, from: &NodeId, to: &NodeId, request_id: &str) -> String }
```

## Visibility

- `pub`

## Docstring

Proxy response (transient)

## Source
Lines 236–242 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [node_topic](/crates/oxide-protocol/src/topics/node_topic.md) |
