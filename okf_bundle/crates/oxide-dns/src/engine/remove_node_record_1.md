---
okf_version: "0.2"
type: Function
title: remove_node_record
description: Remove a mesh node record
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/remove_node_record_1
language: rust
---

# remove_node_record

Remove a mesh node record

## Signature

```rust
pub fn remove_node_record(&self, name: &str) -> Result<()>
```

## Visibility

- `pub`

## Docstring

Remove a mesh node record

## Source
Lines 138–143 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
| calls | [Protocol](/crates/oxide-acl/src/rules/Protocol.md) |
