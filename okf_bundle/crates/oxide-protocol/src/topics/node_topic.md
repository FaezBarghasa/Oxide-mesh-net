---
okf_version: "0.2"
type: Function
title: node_topic
description: Node-scoped topic
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/node_topic
language: rust
---

# node_topic

Node-scoped topic

## Signature

```rust
pub fn node_topic(mesh: &MeshName, node: &NodeId, parts: &[&str]) -> String
```

## Visibility

- `pub`

## Docstring

Node-scoped topic

## Source
Lines 38–44 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| calls | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
| called_by | [accept](/crates/oxide-protocol/src/topics/accept.md) |
| called_by | [advertise](/crates/oxide-protocol/src/topics/advertise.md) |
| called_by | [announce](/crates/oxide-protocol/src/topics/announce.md) |
| called_by | [chunk](/crates/oxide-protocol/src/topics/chunk.md) |
| called_by | [data](/crates/oxide-protocol/src/topics/data.md) |
| called_by | [heartbeat](/crates/oxide-protocol/src/topics/heartbeat.md) |
| called_by | [hole_punch](/crates/oxide-protocol/src/topics/hole_punch.md) |
| called_by | [ice_candidate](/crates/oxide-protocol/src/topics/ice_candidate.md) |
| called_by | [identity](/crates/oxide-protocol/src/topics/identity.md) |
| called_by | [lwt](/crates/oxide-protocol/src/topics/lwt.md) |
| called_by | [metadata](/crates/oxide-protocol/src/topics/metadata.md) |
| called_by | [nat_info](/crates/oxide-protocol/src/topics/nat_info.md) |
| called_by | [node_events](/crates/oxide-protocol/src/topics/node_events.md) |
| called_by | [node_override](/crates/oxide-protocol/src/topics/node_override.md) |
| called_by | [offer](/crates/oxide-protocol/src/topics/offer.md) |
| called_by | [query](/crates/oxide-protocol/src/topics/query.md) |
| called_by | [relay_request](/crates/oxide-protocol/src/topics/relay_request.md) |
| called_by | [request](/crates/oxide-protocol/src/topics/request.md) |
| called_by | [response](/crates/oxide-protocol/src/topics/response.md) |
| called_by | [session](/crates/oxide-protocol/src/topics/session.md) |
| called_by | [withdraw](/crates/oxide-protocol/src/topics/withdraw.md) |
