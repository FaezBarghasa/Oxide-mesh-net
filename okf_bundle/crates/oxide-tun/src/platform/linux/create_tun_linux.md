---
okf_version: "0.2"
type: Function
title: create_tun_linux
description: Create Linux TUN device
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/create_tun_linux
language: rust
---

# create_tun_linux

Create Linux TUN device

## Signature

```rust
pub fn create_tun_linux(config: &TunConfig) -> Result<Box<dyn TunDevice>>
```

## Visibility

- `pub`

## Docstring

Create Linux TUN device

## Source
Lines 481–484 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| called_by | [create_tun](/crates/oxide-tun/src/platform/create_tun.md) |
