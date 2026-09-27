---
okf_version: "0.2"
type: Class
title: PortHopperConfig
description: Port Hopper Configuration
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper/PortHopperConfig
language: rust
---

# PortHopperConfig

Port Hopper Configuration

## Signature

```rust
pub struct PortHopperConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Port Hopper Configuration
[derive(Debug, Clone)]

## Methods

- `seed`
- `min_port`
- `max_port`
- `hop_interval_secs`
- `overlap_window_secs`

## Source
Lines 9–15 in `crates/oxide-transport/src/porthopper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [porthopper](/crates/oxide-transport/src/porthopper.md) |
