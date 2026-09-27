---
okf_version: "0.2"
type: Function
title: epoch_at
description: Compute current epoch from UNIX timestamp
resource: crates/oxide-transport/src/porthopper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:58Z"
concept_id: crates/oxide-transport/src/porthopper/epoch_at
language: rust
---

# epoch_at

Compute current epoch from UNIX timestamp

## Signature

```rust
impl PortHopper { pub fn epoch_at(&self, unix_timestamp_secs: u64) -> u64 }
```

## Visibility

- `pub`

## Docstring

Compute current epoch from UNIX timestamp
[inline]

## Source
Lines 56–58 in `crates/oxide-transport/src/porthopper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [porthopper](/crates/oxide-transport/src/porthopper.md) |
