---
okf_version: "0.2"
type: Function
title: all_routes
description: Return all active routes in the table
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/all_routes
language: rust
---

# all_routes

Return all active routes in the table

## Signature

```rust
impl RadixRoutingTable { pub fn all_routes(&self) -> Vec<RouteEntry> }
```

## Visibility

- `pub`

## Docstring

Return all active routes in the table

## Source
Lines 91–96 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
