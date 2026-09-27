---
okf_version: "0.2"
type: Function
title: new_surrealkv
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/new_surrealkv_1
language: rust
---

# new_surrealkv

## Signature

```rust
pub fn new_surrealkv(
            path: &std::path::Path,
            ns: &str,
            db_name: &str,
        ) -> Result<Self>
```

## Visibility

- `pub`

## Source
Lines 302–321 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| calls | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
