---
okf_version: "0.2"
type: Module
title: circuit_breaker
description: "Transport Circuit Breaker & Actix Web WSS Fallback Mechanism"
resource: crates/oxide-transport/src/circuit_breaker.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/src/circuit_breaker
language: rust
---

# circuit_breaker

Transport Circuit Breaker & Actix Web WSS Fallback Mechanism

## Docstring

Transport Circuit Breaker & Actix Web WSS Fallback Mechanism

Continuously monitors UDP packet loss over a sliding window. If UDP is completely blackholed
(>85% loss for >5s), trips the circuit breaker to fall back to an Actix Web TLS 1.3 WebSocket
reverse-tunnel (/ws/v1/telemetry) without dropping active overlay TCP connections.

## Relationships

| Type | Target |
|------|--------|
| related | [CircuitStatus](/crates/oxide-transport/src/circuit_breaker/CircuitStatus.md) |
| related | [CircuitBreakerConfig](/crates/oxide-transport/src/circuit_breaker/CircuitBreakerConfig.md) |
| related | [default](/crates/oxide-transport/src/circuit_breaker/default.md) |
| related | [default](/crates/oxide-transport/src/circuit_breaker/default.md) |
| related | [TransportCircuitBreaker](/crates/oxide-transport/src/circuit_breaker/TransportCircuitBreaker.md) |
| related | [new](/crates/oxide-transport/src/circuit_breaker/new.md) |
| related | [status](/crates/oxide-transport/src/circuit_breaker/status.md) |
| related | [record_sample](/crates/oxide-transport/src/circuit_breaker/record_sample.md) |
| related | [record_recovery_ping](/crates/oxide-transport/src/circuit_breaker/record_recovery_ping.md) |
| related | [new](/crates/oxide-transport/src/circuit_breaker/new.md) |
| related | [status](/crates/oxide-transport/src/circuit_breaker/status.md) |
| related | [record_sample](/crates/oxide-transport/src/circuit_breaker/record_sample.md) |
| related | [record_recovery_ping](/crates/oxide-transport/src/circuit_breaker/record_recovery_ping.md) |
| related | [test_circuit_breaker_trips_to_fallback_and_recovers](/crates/oxide-transport/src/circuit_breaker/test_circuit_breaker_trips_to_fallback_and_recovers.md) |
