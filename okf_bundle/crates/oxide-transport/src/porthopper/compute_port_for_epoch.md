---
okf_version: "0.2"
type: Function
title: compute_port_for_epoch
description: Compute deterministic pseudo-random port for a given epoch
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper/compute_port_for_epoch
language: rust
---

# compute_port_for_epoch

Compute deterministic pseudo-random port for a given epoch

## Signature

```rust
impl PortHopper { pub fn compute_port_for_epoch(&self, epoch: u64) -> u16 }
```

## Visibility

- `pub`

## Docstring

Compute deterministic pseudo-random port for a given epoch

## Source
Lines 41–52 in `crates/oxide-transport/src/porthopper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [porthopper](/crates/oxide-transport/src/porthopper.md) |
