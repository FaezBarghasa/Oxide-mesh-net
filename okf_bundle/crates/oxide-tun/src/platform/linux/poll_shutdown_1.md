---
okf_version: "0.2"
type: Function
title: poll_shutdown
resource: crates/oxide-tun/src/platform/linux.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/linux/poll_shutdown_1
language: rust
---

# poll_shutdown

## Signature

```rust
fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>>
```

## Source
Lines 444–449 in `crates/oxide-tun/src/platform/linux.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linux](/crates/oxide-tun/src/platform/linux.md) |
