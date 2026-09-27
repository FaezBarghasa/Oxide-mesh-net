---
okf_version: "0.2"
type: Function
title: remediate
description: Re-inject MagicDNS loopback and domain routing into host resolver
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog/remediate
language: rust
---

# remediate

Re-inject MagicDNS loopback and domain routing into host resolver

## Signature

```rust
impl DnsWatchdog { pub fn remediate(&mut self) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Re-inject MagicDNS loopback and domain routing into host resolver

## Source
Lines 104–135 in `crates/oxide-dns/src/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-dns/src/watchdog.md) |
