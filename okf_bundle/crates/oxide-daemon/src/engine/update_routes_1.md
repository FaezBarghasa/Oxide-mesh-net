---
okf_version: "0.2"
type: Function
title: update_routes
description: Update routing table atomically
resource: crates/oxide-daemon/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/engine/update_routes_1
language: rust
---

# update_routes

Update routing table atomically

## Signature

```rust
pub fn update_routes(&self, table: RadixRoutingTable)
```

## Visibility

- `pub`

## Docstring

Update routing table atomically

## Source
Lines 159–161 in `crates/oxide-daemon/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-daemon/src/engine.md) |
