---
okf_version: "0.2"
type: Function
title: poll_flush
resource: crates/oxide-tun/src/platform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/poll_flush
language: rust
---

# poll_flush

## Signature

```rust
impl AsyncFd { fn poll_flush(self: std::pin::Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> }
```

## Source
Lines 166–168 in `crates/oxide-tun/src/platform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [platform](/crates/oxide-tun/src/platform.md) |
