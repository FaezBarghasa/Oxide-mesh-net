---
okf_version: "0.2"
type: Function
title: with_owner
description: Set owner (Linux only)
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/with_owner_1
language: rust
---

# with_owner

Set owner (Linux only)

## Signature

```rust
pub fn with_owner(mut self, uid: Option<u32>, gid: Option<u32>) -> Self
```

## Visibility

- `pub`

## Docstring

Set owner (Linux only)

## Source
Lines 120–124 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
