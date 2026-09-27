---
okf_version: "0.2"
type: Class
title: PresenceAnnounce
description: Node presence announcement payload
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/PresenceAnnounce
language: rust
---

# PresenceAnnounce

Node presence announcement payload

## Signature

```rust
pub struct PresenceAnnounce
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Node presence announcement payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `identity_pub`
- `session_pub`
- `endpoints`
- `capabilities`
- `overlay_ips`
- `advertised_routes`
- `timestamp`
- `version`

## Source
Lines 269–279 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
