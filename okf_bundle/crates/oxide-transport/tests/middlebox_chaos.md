---
okf_version: "0.2"
type: Module
title: middlebox_chaos
description: "Middlebox Chaos & Adversarial Network Condition Tests"
resource: crates/oxide-transport/tests/middlebox_chaos.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-transport/tests/middlebox_chaos
language: rust
---

# middlebox_chaos

Middlebox Chaos & Adversarial Network Condition Tests

## Docstring

Middlebox Chaos & Adversarial Network Condition Tests

Simulates middleboxes injecting 30% synthetic loss, 120ms random jitter, and MTU clamping
down to 1280 bytes, verifying that DPLPMTUD, MSS clamping, and circuit breaking prevent stalls.

## Relationships

| Type | Target |
|------|--------|
| related | [test_middlebox_chaos_mtu_downstep_drill](/crates/oxide-transport/tests/middlebox_chaos/test_middlebox_chaos_mtu_downstep_drill.md) |
| related | [test_middlebox_chaos_total_udp_blackout_and_failover](/crates/oxide-transport/tests/middlebox_chaos/test_middlebox_chaos_total_udp_blackout_and_failover.md) |
| related | [test_clock_skew_port_hopping_overlap](/crates/oxide-transport/tests/middlebox_chaos/test_clock_skew_port_hopping_overlap.md) |
