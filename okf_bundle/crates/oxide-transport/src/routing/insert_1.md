---
okf_version: "0.2"
type: Function
title: insert
description: Insert or replace a route entry and sort by longest prefix length descending
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/insert_1
language: rust
---

# insert

Insert or replace a route entry and sort by longest prefix length descending

## Signature

```rust
pub fn insert(&mut self, prefix: OverlayPrefix, target: RouteTarget)
```

## Visibility

- `pub`

## Docstring

Insert or replace a route entry and sort by longest prefix length descending

## Source
Lines 56–72 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
