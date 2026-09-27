---
okf_version: "0.2"
type: Function
title: poll_read
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/poll_read_1
language: rust
---

# poll_read

## Signature

```rust
fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>>
```

## Source
Lines 387–412 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
