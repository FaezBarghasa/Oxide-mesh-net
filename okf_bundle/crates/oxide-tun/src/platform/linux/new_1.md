---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(config: &TunConfig) -> Result<Self>
```

## Visibility

- `pub`

## Source
Lines 66–98 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| calls | [create_tun_fd](/crates/oxide-tun/src/platform/linux/create_tun_fd.md) |
