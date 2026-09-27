---
okf_version: "0.2"
type: Function
title: create_tun_fd
description: Create a TUN file descriptor
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/create_tun_fd
language: rust
---

# create_tun_fd

Create a TUN file descriptor

## Signature

```rust
fn create_tun_fd(name: &str, is_main: bool, multi_queue: bool) -> Result<OwnedFd>
```

## Docstring

Create a TUN file descriptor

## Source
Lines 453–478 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
| calls | [set_ifr_name](/crates/oxide-tun/src/platform/linux/set_ifr_name.md) |
| called_by | [new](/crates/oxide-tun/src/platform/linux/new.md) |
