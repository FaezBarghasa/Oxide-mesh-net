---
okf_version: "0.2"
type: Function
title: poll_shutdown
resource: crates/oxide-tun/src/platform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/poll_shutdown_1
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
Lines 170–175 in `crates/oxide-tun/src/platform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [platform](/crates/oxide-tun/src/platform.md) |
