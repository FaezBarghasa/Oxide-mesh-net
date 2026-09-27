---
okf_version: "0.2"
type: Function
title: record_sample
description: Record a packet transmission outcome
resource: crates/oxide-transport/src/circuit_breaker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/circuit_breaker/record_sample
language: rust
---

# record_sample

Record a packet transmission outcome

## Signature

```rust
impl TransportCircuitBreaker { pub fn record_sample(&mut self, delivered: bool, now: Instant) }
```

## Visibility

- `pub`

## Docstring

Record a packet transmission outcome

## Source
Lines 70–95 in `crates/oxide-transport/src/circuit_breaker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circuit_breaker](/crates/oxide-transport/src/circuit_breaker.md) |
