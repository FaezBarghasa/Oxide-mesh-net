---
okf_version: "0.2"
type: Class
title: Args
resource: crates/oxide-daemon/src/main.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/main/Args
language: rust
---

# Args

## Signature

```rust
struct Args
```

## Decorators

- `derive(Parser, Debug)`
- `command(
    name = "oxide-daemon",
    about = "High-performance background daemon for oxide-mesh-net"
)`

## Methods

- `config`
- `tun`
- `mtu`
- `port`
- `socket`
- `no_dns`
- `no_mss_clamp`

## Source
Lines 18–46 in `crates/oxide-daemon/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/crates/oxide-daemon/src/main.md) |
