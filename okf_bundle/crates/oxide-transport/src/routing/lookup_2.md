---
okf_version: "0.2"
type: Function
title: lookup
description: Fast-path lookup combining L1 thread-local cache and lock-free RCU load
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing/lookup_2
language: rust
---

# lookup

Fast-path lookup combining L1 thread-local cache and lock-free RCU load

## Signature

```rust
impl RcuRouter { pub fn lookup(&self, ip: OverlayIp, l1: &mut L1DirectMappedCache) -> Option<RouteTarget> }
```

## Visibility

- `pub`

## Docstring

Fast-path lookup combining L1 thread-local cache and lock-free RCU load
[inline]

## Source
Lines 212–224 in `crates/oxide-transport/src/routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routing](/crates/oxide-transport/src/routing.md) |
