---
okf_version: "0.2"
type: Function
title: from
resource: crates/oxide-transport/src/error.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/error/from_2
language: rust
---

# from

## Signature

```rust
impl TransportError { fn from(err: tokio::sync::oneshot::error::RecvError) -> Self }
```

## Source
Lines 60–62 in `crates/oxide-transport/src/error.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [error](/crates/oxide-transport/src/error.md) |
