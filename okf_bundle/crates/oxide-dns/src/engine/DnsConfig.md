---
okf_version: "0.2"
type: Class
title: DnsConfig
description: MagicDNS configuration
resource: crates/oxide-dns/src/engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-25T20:47:21Z"
concept_id: crates/oxide-dns/src/engine/DnsConfig
language: rust
---

# DnsConfig

MagicDNS configuration

## Signature

```rust
pub struct DnsConfig
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

MagicDNS configuration
[derive(Debug, Clone)]

## Methods

- `listen_addr`
- `mesh_tld`
- `upstream_resolvers`
- `dnssec`
- `cache_ttl`
- `split_horizon`

## Source
Lines 21–34 in `crates/oxide-dns/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-dns/src/engine.md) |
