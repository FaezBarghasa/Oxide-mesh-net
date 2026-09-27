---
okf_version: "0.2"
type: Module
title: watchdog
description: "Native OS DNS Interception & Self-Healing Watchdogs"
resource: crates/oxide-dns/src/watchdog.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-dns"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-dns/src/watchdog
language: rust
---

# watchdog

Native OS DNS Interception & Self-Healing Watchdogs

## Docstring

Native OS DNS Interception & Self-Healing Watchdogs

Enforces active `.oxide` MagicDNS resolution and loopback resolver (100.100.100.100)
ownership across OS restarts, NetworkManager resets, and VPN interface toggles.

## Relationships

| Type | Target |
|------|--------|
| related | [OsDnsPlatform](/crates/oxide-dns/src/watchdog/OsDnsPlatform.md) |
| related | [DnsWatchdogConfig](/crates/oxide-dns/src/watchdog/DnsWatchdogConfig.md) |
| related | [default](/crates/oxide-dns/src/watchdog/default.md) |
| related | [default](/crates/oxide-dns/src/watchdog/default.md) |
| related | [DnsWatchdog](/crates/oxide-dns/src/watchdog/DnsWatchdog.md) |
| related | [new](/crates/oxide-dns/src/watchdog/new.md) |
| related | [detect_platform](/crates/oxide-dns/src/watchdog/detect_platform.md) |
| related | [verify_active](/crates/oxide-dns/src/watchdog/verify_active.md) |
| related | [remediate](/crates/oxide-dns/src/watchdog/remediate.md) |
| related | [run_loop](/crates/oxide-dns/src/watchdog/run_loop.md) |
| related | [remediation_count](/crates/oxide-dns/src/watchdog/remediation_count.md) |
| related | [new](/crates/oxide-dns/src/watchdog/new.md) |
| related | [detect_platform](/crates/oxide-dns/src/watchdog/detect_platform.md) |
| related | [verify_active](/crates/oxide-dns/src/watchdog/verify_active.md) |
| related | [remediate](/crates/oxide-dns/src/watchdog/remediate.md) |
| related | [run_loop](/crates/oxide-dns/src/watchdog/run_loop.md) |
| related | [remediation_count](/crates/oxide-dns/src/watchdog/remediation_count.md) |
| related | [test_dns_watchdog_detection_and_remediation](/crates/oxide-dns/src/watchdog/test_dns_watchdog_detection_and_remediation.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
