---
okf_version: "0.2"
type: Function
title: create_tun_macos
resource: crates/oxide-tun/src/platform/macos.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/macos/create_tun_macos
language: rust
---

# create_tun_macos

## Signature

```rust
pub fn create_tun_macos(config: &TunConfig) -> Result<Box<dyn TunDevice>>
```

## Visibility

- `pub`

## Source
Lines 195–198 in `crates/oxide-tun/src/platform/macos.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [macos](/crates/oxide-tun/src/platform/macos.md) |
| called_by | [create_tun](/crates/oxide-tun/src/platform/create_tun.md) |
