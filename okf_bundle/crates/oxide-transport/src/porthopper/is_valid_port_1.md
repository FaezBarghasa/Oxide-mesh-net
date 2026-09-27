---
okf_version: "0.2"
type: Function
title: is_valid_port
description: "Return true if a received packet on `dest_port` belongs to the current valid sliding window"
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper/is_valid_port_1
language: rust
---

# is_valid_port

Return true if a received packet on `dest_port` belongs to the current valid sliding window

## Signature

```rust
pub fn is_valid_port(&self, unix_timestamp_secs: u64, dest_port: u16) -> bool
```

## Visibility

- `pub`

## Docstring

Return true if a received packet on `dest_port` belongs to the current valid sliding window

## Source
Lines 70–73 in `crates/oxide-transport/src/porthopper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [porthopper](/crates/oxide-transport/src/porthopper.md) |
