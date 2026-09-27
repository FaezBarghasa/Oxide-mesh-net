---
okf_version: "0.2"
type: Function
title: create_tun
description: Create a new TUN device based on platform
resource: crates/oxide-tun/src/platform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/create_tun
language: rust
---

# create_tun

Create a new TUN device based on platform

## Signature

```rust
pub fn create_tun(config: &TunConfig) -> Result<Box<dyn TunDevice>>
```

## Visibility

- `pub`

## Docstring

Create a new TUN device based on platform

## Source
Lines 72–91 in `crates/oxide-tun/src/platform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [platform](/crates/oxide-tun/src/platform.md) |
| calls | [create_tun_linux](/crates/oxide-tun/src/platform/linux/create_tun_linux.md) |
| calls | [create_tun_windows](/crates/oxide-tun/src/platform/windows/create_tun_windows.md) |
| calls | [create_tun_macos](/crates/oxide-tun/src/platform/macos/create_tun_macos.md) |
