---
okf_version: "0.2"
type: Class
title: TelemetrySnapshot
description: "[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-ui/src/models/delta.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T17:07:22Z"
concept_id: crates/oxide-ui/src/models/delta/TelemetrySnapshot
language: rust
---

# TelemetrySnapshot

[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub struct TelemetrySnapshot
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `ingress_bytes_sec`
- `egress_bytes_sec`
- `pps_in`
- `pps_out`
- `current_mtu`
- `overlay_ipv4`
- `overlay_ipv6`
- `active_threads`
- `nat_type`
- `throughput_history`
- `latency_history`

## Source
Lines 32–44 in `crates/oxide-ui/src/models/delta.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [delta](/crates/oxide-ui/src/models/delta.md) |
