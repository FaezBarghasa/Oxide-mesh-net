---
okf_version: "0.2"
type: Function
title: create_tun_windows
description: "AsyncRead/AsyncWrite would be implemented using Wintun's ring buffer API"
resource: crates/oxide-tun/src/platform/windows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/windows/create_tun_windows
language: rust
---

# create_tun_windows

AsyncRead/AsyncWrite would be implemented using Wintun's ring buffer API

## Signature

```rust
pub fn create_tun_windows(config: &TunConfig) -> Result<Box<dyn TunDevice>>
```

## Visibility

- `pub`

## Docstring

AsyncRead/AsyncWrite would be implemented using Wintun's ring buffer API
This is a placeholder

## Source
Lines 145–148 in `crates/oxide-tun/src/platform/windows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [windows](/crates/oxide-tun/src/platform/windows.md) |
| called_by | [create_tun](/crates/oxide-tun/src/platform/create_tun.md) |
