---
okf_version: "0.2"
type: Function
title: compute_inet_checksum
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:41Z"
concept_id: crates/oxide-tun/src/mss/compute_inet_checksum
language: rust
---

# compute_inet_checksum

## Signature

```rust
fn compute_inet_checksum(data: &[u8]) -> u16
```

## Source
Lines 215–229 in `crates/oxide-tun/src/mss.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mss](/crates/oxide-tun/src/mss.md) |
| called_by | [test_rfc1624_checksum_incremental_update](/crates/oxide-tun/src/mss/test_rfc1624_checksum_incremental_update.md) |
