---
okf_version: "0.2"
type: Function
title: handle_mesh_query
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/handle_mesh_query_1
language: rust
---

# handle_mesh_query

## Signature

```rust
fn handle_mesh_query(
        &self,
        request: &Request,
        mut response_handler: R,
        name: &Name,
        record_type: RecordType,
    ) -> ResponseInfo
```

## Type Parameters

- `R: ResponseHandler`

## Source
Lines 245–279 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
