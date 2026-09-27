---
okf_version: "0.2"
type: Function
title: handle_subnet_query
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/handle_subnet_query
language: rust
---

# handle_subnet_query

## Signature

```rust
impl MagicDnsRequestHandler { fn handle_subnet_query(
        &self,
        request: &Request,
        response_handler: R,
        name: &Name,
        record_type: RecordType,
    ) -> ResponseInfo }
```

## Type Parameters

- `R: ResponseHandler`

## Source
Lines 281–290 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
