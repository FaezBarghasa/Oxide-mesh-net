---
okf_version: "0.2"
type: Function
title: active_ports
description: "Return active listening port triple [P_prev, P_current, P_next] for dual-epoch sliding"
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper/active_ports_1
language: rust
---

# active_ports

Return active listening port triple [P_prev, P_current, P_next] for dual-epoch sliding

## Signature

```rust
pub fn active_ports(&self, unix_timestamp_secs: u64) -> (u16, u16, u16)
```

## Visibility

- `pub`

## Docstring

Return active listening port triple [P_prev, P_current, P_next] for dual-epoch sliding

## Source
Lines 61–67 in `crates/oxide-transport/src/porthopper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [porthopper](/crates/oxide-transport/src/porthopper.md) |
