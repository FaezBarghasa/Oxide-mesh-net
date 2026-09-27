---
okf_version: "0.2"
type: Class
title: RouteEntryDto
description: Routing entry DTO
resource: crates/oxide-daemon/src/ipc/protocol.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/protocol/RouteEntryDto
language: rust
---

# RouteEntryDto

Routing entry DTO

## Signature

```rust
pub struct RouteEntryDto
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Routing entry DTO
[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

## Methods

- `prefix`
- `target_node`
- `pmtu`
- `is_direct`

## Source
Lines 51–56 in `crates/oxide-daemon/src/ipc/protocol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [protocol](/crates/oxide-daemon/src/ipc/protocol.md) |
