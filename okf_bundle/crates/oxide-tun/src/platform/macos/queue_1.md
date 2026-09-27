---
okf_version: "0.2"
type: Function
title: queue
resource: crates/oxide-tun/src/platform/macos.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/platform/macos/queue_1
language: rust
---

# queue

## Signature

```rust
fn queue(&self, index: usize) -> Option<Box<dyn TunQueue>>
```

## Source
Lines 90–96 in `crates/oxide-tun/src/platform/macos.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [macos](/crates/oxide-tun/src/platform/macos.md) |
