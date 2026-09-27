---
okf_version: "0.2"
type: Function
title: zone
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/zone_1
language: rust
---

# zone

## Signature

```rust
fn zone(&self, name: &Name) -> Option<Arc<dyn hickory_server::authority::Authority>>
```

## Source
Lines 190–193 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
