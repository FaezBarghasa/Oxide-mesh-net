---
okf_version: "0.2"
type: Function
title: delete_route
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/delete_route
language: rust
---

# delete_route

## Signature

```rust
impl Storage { pub fn delete_route(&self, prefix: &OverlayPrefix) -> Result<()> }
```

## Visibility

- `pub`

## Source
Lines 173–175 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
