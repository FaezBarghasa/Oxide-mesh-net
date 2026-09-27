---
okf_version: "0.2"
type: Function
title: run_loop
description: Run background watchdog loop
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog/run_loop
language: rust
---

# run_loop

Run background watchdog loop

## Signature

```rust
impl DnsWatchdog { pub fn run_loop(&mut self) }
```

## Visibility

- `pub`

## Docstring

Run background watchdog loop

## Source
Lines 138–152 in `crates/oxide-dns/src/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-dns/src/watchdog.md) |
