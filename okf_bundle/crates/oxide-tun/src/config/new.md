---
okf_version: "0.2"
type: Function
title: new
description: Create a new TUN config with the given name
resource: crates/oxide-tun/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-tun/src/config/new
language: rust
---

# new

Create a new TUN config with the given name

## Signature

```rust
impl TunConfig { pub fn new(name: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new TUN config with the given name

## Source
Lines 50–55 in `crates/oxide-tun/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-tun/src/config.md) |
