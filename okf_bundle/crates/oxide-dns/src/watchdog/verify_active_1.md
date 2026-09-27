---
okf_version: "0.2"
type: Function
title: verify_active
description: Verify whether MagicDNS domain routing is currently active
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog/verify_active_1
language: rust
---

# verify_active

Verify whether MagicDNS domain routing is currently active

## Signature

```rust
pub fn verify_active(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Verify whether MagicDNS domain routing is currently active

## Source
Lines 88–101 in `crates/oxide-dns/src/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-dns/src/watchdog.md) |
