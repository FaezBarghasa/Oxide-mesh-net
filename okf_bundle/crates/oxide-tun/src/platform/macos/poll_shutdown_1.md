---
okf_version: "0.2"
type: Function
title: poll_shutdown
resource: crates/oxide-tun/src/platform/macos.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/macos/poll_shutdown_1
language: rust
---

# poll_shutdown

## Signature

```rust
fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>>
```

## Source
Lines 187–192 in `crates/oxide-tun/src/platform/macos.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [macos](/crates/oxide-tun/src/platform/macos.md) |
