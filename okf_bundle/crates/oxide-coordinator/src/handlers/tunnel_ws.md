---
okf_version: "0.2"
type: Module
title: tunnel_ws
description: Actix Web WebSocket Reverse-Tunnel Fallback Handler (/ws/v1/telemetry)
resource: crates/oxide-coordinator/src/handlers/tunnel_ws.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-coordinator/src/handlers/tunnel_ws
language: rust
---

# tunnel_ws

Actix Web WebSocket Reverse-Tunnel Fallback Handler (/ws/v1/telemetry)

## Docstring

Actix Web WebSocket Reverse-Tunnel Fallback Handler (/ws/v1/telemetry)

Provides seamless fallback data transport disguised as standard browser telemetry
with authentic HTTP headers and binary WebSocket packet encapsulation.

## Relationships

| Type | Target |
|------|--------|
| related | [ws_telemetry_tunnel](/crates/oxide-coordinator/src/handlers/tunnel_ws/ws_telemetry_tunnel.md) |
| related | [configure_tunnel_routes](/crates/oxide-coordinator/src/handlers/tunnel_ws/configure_tunnel_routes.md) |
| related | [test_ws_telemetry_tunnel_route_registration](/crates/oxide-coordinator/src/handlers/tunnel_ws/test_ws_telemetry_tunnel_route_registration.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
