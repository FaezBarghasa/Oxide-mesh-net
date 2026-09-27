---
okf_version: "0.2"
type: Class
title: AppState
description: "[derive(Debug, Clone, PartialEq)]"
resource: crates/oxide-ui/src/state/app_state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-ui/src/state/app_state/AppState
language: rust
---

# AppState

[derive(Debug, Clone, PartialEq)]

## Signature

```rust
pub struct AppState
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, PartialEq)]

## Methods

- `current_view`
- `node_state`
- `is_reconnecting`
- `reconnect_attempts`
- `telemetry`
- `active_peers`
- `selected_peer_diagnostics`
- `is_drawer_open`
- `exit_nodes`
- `advertised_subnets`
- `split_rules`
- `route_test_result`
- `dpi_config`
- `reality_config`
- `frag_config`
- `port_hop_state`
- `device_auth`
- `acl_rules`
- `hsm_status`
- `acl_simulation_result`
- `magic_dns`
- `drop_transfers`
- `active_ssh_sessions`
- `ingress_services`
- `merkle_logs`
- `packet_stream`
- `packet_filter_query`

## Source
Lines 17–57 in `crates/oxide-ui/src/state/app_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [app_state](/crates/oxide-ui/src/state/app_state.md) |
