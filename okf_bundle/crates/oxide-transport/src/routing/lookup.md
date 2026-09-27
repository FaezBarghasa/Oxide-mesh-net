---
okf_version: "0.2"
type: Function
title: lookup
description: Longest Prefix Match lookup
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/lookup
language: rust
---

# lookup

Longest Prefix Match lookup

## Signature

```rust
impl RadixRoutingTable { pub fn lookup(&self, ip: OverlayIp) -> Option<RouteTarget> }
```

## Visibility

- `pub`

## Docstring

Longest Prefix Match lookup
[inline]

## Source
Lines 100–118 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
