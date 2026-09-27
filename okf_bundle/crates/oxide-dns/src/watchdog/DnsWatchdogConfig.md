---
okf_version: "0.2"
type: Class
title: DnsWatchdogConfig
description: Self-healing OS DNS Monitor Configuration
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog/DnsWatchdogConfig
language: rust
---

# DnsWatchdogConfig

Self-healing OS DNS Monitor Configuration

## Signature

```rust
pub struct DnsWatchdogConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Self-healing OS DNS Monitor Configuration
[derive(Debug, Clone)]

## Methods

- `magic_dns_ip`
- `domain_suffix`
- `poll_interval`
- `auto_remediate`

## Source
Lines 26–31 in `crates/oxide-dns/src/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-dns/src/watchdog.md) |
