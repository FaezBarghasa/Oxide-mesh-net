---
okf_version: "0.2"
type: Module
title: mss
description: Dynamic TCP MSS Clamping implementation for IPv4 and IPv6 packets
resource: crates/oxide-tun/src/mss.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-tun"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:44:41Z"
concept_id: crates/oxide-tun/src/mss
language: rust
---

# mss

Dynamic TCP MSS Clamping implementation for IPv4 and IPv6 packets

## Docstring

Dynamic TCP MSS Clamping implementation for IPv4 and IPv6 packets

Enforces path MTU constraints on TCP connections crossing TUN/overlay boundaries
by intercepting TCP SYN and SYN-ACK packets, rewriting the Maximum Segment Size (MSS)
option, and performing RFC 1624 incremental one's complement checksum updates.

## Relationships

| Type | Target |
|------|--------|
| related | [update_checksum_16](/crates/oxide-tun/src/mss/update_checksum_16.md) |
| related | [clamp_tcp_mss](/crates/oxide-tun/src/mss/clamp_tcp_mss.md) |
| related | [clamp_ipv4_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv4_tcp_mss.md) |
| related | [clamp_ipv6_tcp_mss](/crates/oxide-tun/src/mss/clamp_ipv6_tcp_mss.md) |
| related | [clamp_tcp_segment](/crates/oxide-tun/src/mss/clamp_tcp_segment.md) |
| related | [compute_inet_checksum](/crates/oxide-tun/src/mss/compute_inet_checksum.md) |
| related | [test_rfc1624_checksum_incremental_update](/crates/oxide-tun/src/mss/test_rfc1624_checksum_incremental_update.md) |
| related | [test_clamp_ipv4_tcp_syn](/crates/oxide-tun/src/mss/test_clamp_ipv4_tcp_syn.md) |
| related | [test_clamp_unnecessary_mss](/crates/oxide-tun/src/mss/test_clamp_unnecessary_mss.md) |
