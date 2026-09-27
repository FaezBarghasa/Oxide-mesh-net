---
okf_version: "0.2"
type: Class
title: TransportCircuitBreaker
description: Dynamic Transport Circuit Breaker
resource: crates/oxide-transport/src/circuit_breaker.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/circuit_breaker/TransportCircuitBreaker
language: rust
---

# TransportCircuitBreaker

Dynamic Transport Circuit Breaker

## Signature

```rust
pub struct TransportCircuitBreaker
```

## Visibility

- `pub`

## Docstring

Dynamic Transport Circuit Breaker

## Methods

- `config`
- `status`
- `sent_packets`
- `lost_packets`
- `high_loss_since`
- `consecutive_recovery_pings`
- `last_probe_sent`

## Source
Lines 41–50 in `crates/oxide-transport/src/circuit_breaker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circuit_breaker](/crates/oxide-transport/src/circuit_breaker.md) |
