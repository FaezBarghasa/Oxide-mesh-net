---
okf_version: "0.2"
type: Function
title: new_mem
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/new_mem
language: rust
---

# new_mem

## Signature

```rust
impl SurrealStorage { pub fn new_mem(ns: &str, db_name: &str) -> Result<Self> }
```

## Visibility

- `pub`

## Source
Lines 286–300 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
