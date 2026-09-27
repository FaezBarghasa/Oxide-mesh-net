---
okf_version: "0.2"
type: Function
title: topic
description: Build a topic string
resource: crates/oxide-protocol/src/topics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-protocol/src/topics/topic
language: rust
---

# topic

Build a topic string

## Signature

```rust
pub fn topic(parts: &[&str]) -> String
```

## Visibility

- `pub`

## Docstring

Build a topic string

## Source
Lines 22–27 in `crates/oxide-protocol/src/topics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topics](/crates/oxide-protocol/src/topics.md) |
| called_by | [election](/crates/oxide-protocol/src/topics/election.md) |
| called_by | [membership](/crates/oxide-protocol/src/topics/membership.md) |
| called_by | [mesh_topic](/crates/oxide-protocol/src/topics/mesh_topic.md) |
| called_by | [replication](/crates/oxide-protocol/src/topics/replication.md) |
