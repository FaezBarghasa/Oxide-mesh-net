---
okf_version: "0.2"
type: Function
title: update_checksum_16
description: "RFC 1624 Incremental 16-bit One's Complement Checksum Update"
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:41Z"
concept_id: crates/oxide-tun/src/mss/update_checksum_16
language: rust
---

# update_checksum_16

RFC 1624 Incremental 16-bit One's Complement Checksum Update

## Signature

```rust
pub fn update_checksum_16(old_checksum: u16, old_val: u16, new_val: u16) -> u16
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

RFC 1624 Incremental 16-bit One's Complement Checksum Update

Formula: HC' = ~(~HC + ~m + m')
where:
- HC is the original checksum
- m is the original 16-bit word being replaced
- m' is the new 16-bit word
[inline]

## Source
Lines 17–30 in `crates/oxide-tun/src/mss.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mss](/crates/oxide-tun/src/mss.md) |
| called_by | [clamp_tcp_segment](/crates/oxide-tun/src/mss/clamp_tcp_segment.md) |
| called_by | [test_rfc1624_checksum_incremental_update](/crates/oxide-tun/src/mss/test_rfc1624_checksum_incremental_update.md) |
