---
okf_version: "0.2"
type: Function
title: record_recovery_ping
description: Record background UDP recovery ping result while in fallback
resource: crates/oxide-transport/src/circuit_breaker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/circuit_breaker/record_recovery_ping_1
language: rust
---

# record_recovery_ping

Record background UDP recovery ping result while in fallback

## Signature

```rust
pub fn record_recovery_ping(&mut self, success: bool)
```

## Visibility

- `pub`

## Docstring

Record background UDP recovery ping result while in fallback

## Source
Lines 98–112 in `crates/oxide-transport/src/circuit_breaker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circuit_breaker](/crates/oxide-transport/src/circuit_breaker.md) |
