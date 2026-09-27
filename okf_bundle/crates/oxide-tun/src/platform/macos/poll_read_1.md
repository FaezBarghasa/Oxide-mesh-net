---
okf_version: "0.2"
type: Function
title: poll_read
resource: crates/oxide-tun/src/platform/macos.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/macos/poll_read_1
language: rust
---

# poll_read

## Signature

```rust
fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>>
```

## Source
Lines 162–168 in `crates/oxide-tun/src/platform/macos.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [macos](/crates/oxide-tun/src/platform/macos.md) |
