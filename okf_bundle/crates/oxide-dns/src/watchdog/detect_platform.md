---
okf_version: "0.2"
type: Function
title: detect_platform
description: Detect current OS platform mechanism
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog/detect_platform
language: rust
---

# detect_platform

Detect current OS platform mechanism

## Signature

```rust
impl DnsWatchdog { pub fn detect_platform() -> OsDnsPlatform }
```

## Visibility

- `pub`

## Docstring

Detect current OS platform mechanism

## Source
Lines 64–85 in `crates/oxide-dns/src/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-dns/src/watchdog.md) |
