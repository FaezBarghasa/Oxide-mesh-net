---
okf_version: "0.2"
type: Function
title: add_subnet_route
description: Add a subnet router route
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/add_subnet_route_1
language: rust
---

# add_subnet_route

Add a subnet router route

## Signature

```rust
pub fn add_subnet_route(&self, prefix: OverlayPrefix, router_name: &str) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Add a subnet router route

## Source
Lines 146–149 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
