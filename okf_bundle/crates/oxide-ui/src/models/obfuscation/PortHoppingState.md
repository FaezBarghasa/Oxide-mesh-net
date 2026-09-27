---
okf_version: "0.2"
type: Class
title: PortHoppingState
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
resource: crates/oxide-ui/src/models/obfuscation.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ui"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-26T16:51:53Z"
concept_id: crates/oxide-ui/src/models/obfuscation/PortHoppingState
language: rust
---

# PortHoppingState

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Signature

```rust
pub struct PortHoppingState
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `enabled`
- `port_range_start`
- `port_range_end`
- `hop_interval_secs`
- `current_port`
- `next_port_candidate`
- `seconds_to_next_hop`
- `emergency_wss_fallback_active`
- `enforce_wss_only`

## Source
Lines 35–45 in `crates/oxide-ui/src/models/obfuscation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [obfuscation](/crates/oxide-ui/src/models/obfuscation.md) |
