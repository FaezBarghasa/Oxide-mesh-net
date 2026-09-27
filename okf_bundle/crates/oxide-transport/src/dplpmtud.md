---
okf_version: "0.2"
type: Module
title: dplpmtud
description: RFC 8899 Datagram Packetization Layer Path MTU Discovery (DPLPMTUD)
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud
language: rust
---

# dplpmtud

RFC 8899 Datagram Packetization Layer Path MTU Discovery (DPLPMTUD)

## Docstring

RFC 8899 Datagram Packetization Layer Path MTU Discovery (DPLPMTUD)

Provides automated and continuous path capacity measurement between overlay mesh peers
over QUIC datagrams, dynamically adjusting session PMTU and avoiding blackholing.

## Relationships

| Type | Target |
|------|--------|
| related | [DplpmtudPhase](/crates/oxide-transport/src/dplpmtud/DplpmtudPhase.md) |
| related | [DplpmtudConfig](/crates/oxide-transport/src/dplpmtud/DplpmtudConfig.md) |
| related | [default](/crates/oxide-transport/src/dplpmtud/default.md) |
| related | [default](/crates/oxide-transport/src/dplpmtud/default.md) |
| related | [DplpmtudEngine](/crates/oxide-transport/src/dplpmtud/DplpmtudEngine.md) |
| related | [new](/crates/oxide-transport/src/dplpmtud/new.md) |
| related | [confirmed_pmtu](/crates/oxide-transport/src/dplpmtud/confirmed_pmtu.md) |
| related | [phase](/crates/oxide-transport/src/dplpmtud/phase.md) |
| related | [poll_probe](/crates/oxide-transport/src/dplpmtud/poll_probe.md) |
| related | [on_probe_acked](/crates/oxide-transport/src/dplpmtud/on_probe_acked.md) |
| related | [on_probe_lost](/crates/oxide-transport/src/dplpmtud/on_probe_lost.md) |
| related | [on_packet_too_big](/crates/oxide-transport/src/dplpmtud/on_packet_too_big.md) |
| related | [new](/crates/oxide-transport/src/dplpmtud/new.md) |
| related | [confirmed_pmtu](/crates/oxide-transport/src/dplpmtud/confirmed_pmtu.md) |
| related | [phase](/crates/oxide-transport/src/dplpmtud/phase.md) |
| related | [poll_probe](/crates/oxide-transport/src/dplpmtud/poll_probe.md) |
| related | [on_probe_acked](/crates/oxide-transport/src/dplpmtud/on_probe_acked.md) |
| related | [on_probe_lost](/crates/oxide-transport/src/dplpmtud/on_probe_lost.md) |
| related | [on_packet_too_big](/crates/oxide-transport/src/dplpmtud/on_packet_too_big.md) |
| related | [test_dplpmtud_lifecycle](/crates/oxide-transport/src/dplpmtud/test_dplpmtud_lifecycle.md) |
