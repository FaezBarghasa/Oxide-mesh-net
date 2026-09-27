---
okf_version: "0.2"
type: Class
title: PacketStreamEntry
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/audit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:53:23Z"
concept_id: crates/oxide-ui/src/models/audit/PacketStreamEntry
language: rust
---

# PacketStreamEntry

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct PacketStreamEntry
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `timestamp_us`
- `src_ip`
- `dst_ip`
- `protocol`
- `src_port`
- `dst_port`
- `payload_bytes`
- `is_dropped`
- `drop_reason`

## Source
Lines 17–27 in `crates/oxide-ui/src/models/audit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [audit](/crates/oxide-ui/src/models/audit.md) |
