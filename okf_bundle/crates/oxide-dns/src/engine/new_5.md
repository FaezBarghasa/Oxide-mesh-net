---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/new_5
language: rust
---

# new

## Signature

```rust
pub fn new(mesh_dns: SocketAddr, upstream_resolvers: Vec<String>, mesh_tld: String) -> Result<Self>
```

## Visibility

- `pub`

## Source
Lines 334–349 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
