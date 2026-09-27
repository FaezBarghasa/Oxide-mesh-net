---
okf_version: "0.2"
type: Function
title: add_node_record
description: Add a mesh node record
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/add_node_record
language: rust
---

# add_node_record

Add a mesh node record

## Signature

```rust
impl MagicDns { pub fn add_node_record(&self, name: &str, ip: OverlayIp, ttl: u32) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Add a mesh node record

## Source
Lines 119–135 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
