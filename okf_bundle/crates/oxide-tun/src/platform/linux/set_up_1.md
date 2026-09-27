---
okf_version: "0.2"
type: Function
title: set_up
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/set_up_1
language: rust
---

# set_up

## Signature

```rust
fn set_up(&self, up: bool) -> Result<()>
```

## Source
Lines 193–226 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| calls | [set_ifr_name](/crates/oxide-tun/src/platform/linux/set_ifr_name.md) |
