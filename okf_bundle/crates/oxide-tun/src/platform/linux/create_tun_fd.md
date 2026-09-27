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
