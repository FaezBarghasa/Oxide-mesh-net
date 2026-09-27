---
okf_version: "0.2"
type: Function
title: forward_upstream
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/forward_upstream_1
language: rust
---

# forward_upstream

## Signature

```rust
fn forward_upstream(
        &self,
        request: &Request,
        response_handler: R,
    ) -> ResponseInfo
```

## Type Parameters

- `R: ResponseHandler`

## Source
Lines 292–323 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
