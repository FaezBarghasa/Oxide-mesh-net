---
okf_version: "0.2"
type: Class
title: RouteTarget
description: Target destination for routed traffic
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/RouteTarget
language: rust
---

# RouteTarget

Target destination for routed traffic

## Signature

```rust
pub struct RouteTarget
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Target destination for routed traffic
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `node_id`
- `pmtu`
- `is_direct`

## Source
Lines 12–16 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
