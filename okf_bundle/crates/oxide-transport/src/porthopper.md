---
okf_version: "0.2"
type: Module
title: porthopper
description: Dual-Epoch PRNG Port Hopping Synchronization
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper
language: rust
---

# porthopper

Dual-Epoch PRNG Port Hopping Synchronization

## Docstring

Dual-Epoch PRNG Port Hopping Synchronization

Provides clock-drift resilient port hopping across dynamic UDP port ranges (e.g. 25000..45000),
computing concurrent active listening ports across [P_prev, P_current, P_next] to eliminate
packet drop transitions under clock skew.

## Relationships

| Type | Target |
|------|--------|
| related | [PortHopperConfig](/crates/oxide-transport/src/porthopper/PortHopperConfig.md) |
| related | [default](/crates/oxide-transport/src/porthopper/default.md) |
| related | [default](/crates/oxide-transport/src/porthopper/default.md) |
| related | [PortHopper](/crates/oxide-transport/src/porthopper/PortHopper.md) |
| related | [new](/crates/oxide-transport/src/porthopper/new.md) |
| related | [compute_port_for_epoch](/crates/oxide-transport/src/porthopper/compute_port_for_epoch.md) |
| related | [epoch_at](/crates/oxide-transport/src/porthopper/epoch_at.md) |
| related | [active_ports](/crates/oxide-transport/src/porthopper/active_ports.md) |
| related | [is_valid_port](/crates/oxide-transport/src/porthopper/is_valid_port.md) |
| related | [new](/crates/oxide-transport/src/porthopper/new.md) |
| related | [compute_port_for_epoch](/crates/oxide-transport/src/porthopper/compute_port_for_epoch.md) |
| related | [epoch_at](/crates/oxide-transport/src/porthopper/epoch_at.md) |
| related | [active_ports](/crates/oxide-transport/src/porthopper/active_ports.md) |
| related | [is_valid_port](/crates/oxide-transport/src/porthopper/is_valid_port.md) |
| related | [test_port_hopper_distribution_and_bounds](/crates/oxide-transport/src/porthopper/test_port_hopper_distribution_and_bounds.md) |
| related | [test_active_ports_sliding_window](/crates/oxide-transport/src/porthopper/test_active_ports_sliding_window.md) |
