---
okf_version: "0.2"
type: Module
title: engine
description: "Master Data Plane & Coordination Engine for oxide-daemon"
resource: crates/oxide-daemon/src/engine.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/engine
language: rust
---

# engine

Master Data Plane & Coordination Engine for oxide-daemon

## Docstring

Master Data Plane & Coordination Engine for oxide-daemon

Orchestrates multi-queue TUN interfaces, TCP MSS clamping, lock-free RCU Radix routing,
DPLPMTUD probing, port hopping synchronization, and OS DNS self-healing.

## Relationships

| Type | Target |
|------|--------|
| related | [DaemonEngine](/crates/oxide-daemon/src/engine/DaemonEngine.md) |
| related | [new](/crates/oxide-daemon/src/engine/new.md) |
| related | [process_tun_packet](/crates/oxide-daemon/src/engine/process_tun_packet.md) |
| related | [start](/crates/oxide-daemon/src/engine/start.md) |
| related | [update_routes](/crates/oxide-daemon/src/engine/update_routes.md) |
| related | [config](/crates/oxide-daemon/src/engine/config.md) |
| related | [new](/crates/oxide-daemon/src/engine/new.md) |
| related | [process_tun_packet](/crates/oxide-daemon/src/engine/process_tun_packet.md) |
| related | [start](/crates/oxide-daemon/src/engine/start.md) |
| related | [update_routes](/crates/oxide-daemon/src/engine/update_routes.md) |
| related | [config](/crates/oxide-daemon/src/engine/config.md) |
| related | [handle_request](/crates/oxide-daemon/src/engine/handle_request.md) |
| related | [handle_request](/crates/oxide-daemon/src/engine/handle_request.md) |
| related | [test_daemon_packet_processing_pipeline](/crates/oxide-daemon/src/engine/test_daemon_packet_processing_pipeline.md) |
| related | [test_daemon_ipc_handler](/crates/oxide-daemon/src/engine/test_daemon_ipc_handler.md) |
| related | [instant](/_dependencies/cargo/instant.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
