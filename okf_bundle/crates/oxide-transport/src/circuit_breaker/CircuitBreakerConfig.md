---
okf_version: "0.2"
type: Class
title: CircuitBreakerConfig
description: Circuit Breaker Configuration
resource: crates/oxide-transport/src/circuit_breaker.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/circuit_breaker/CircuitBreakerConfig
language: rust
---

# CircuitBreakerConfig

Circuit Breaker Configuration

## Signature

```rust
pub struct CircuitBreakerConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Circuit Breaker Configuration
[derive(Debug, Clone)]

## Methods

- `loss_threshold_percent`
- `trigger_duration`
- `recovery_success_threshold`
- `probe_interval`

## Source
Lines 22–27 in `crates/oxide-transport/src/circuit_breaker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circuit_breaker](/crates/oxide-transport/src/circuit_breaker.md) |
