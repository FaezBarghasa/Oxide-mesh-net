---
okf_version: "0.2"
type: Function
title: poll
resource: crates/oxide-tun/src/platform/windows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/windows/poll_1
language: rust
---

# poll

## Signature

```rust
fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output>
```

## Source
Lines 133–139 in `crates/oxide-tun/src/platform/windows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [windows](/crates/oxide-tun/src/platform/windows.md) |
