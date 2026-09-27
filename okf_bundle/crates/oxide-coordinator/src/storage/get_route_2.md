---
okf_version: "0.2"
type: Function
title: get_route
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/get_route_2
language: rust
---

# get_route

## Signature

```rust
impl SurrealStorage { fn get_route(&self, prefix: &OverlayPrefix) -> Result<Option<RouteAdvertisement>> }
```

## Source
Lines 487–517 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
