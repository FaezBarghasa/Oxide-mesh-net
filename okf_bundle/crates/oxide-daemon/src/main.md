---
okf_version: "0.2"
type: Module
title: src
description: Oxide Mesh Net Daemon Binary
resource: crates/oxide-daemon/src/main.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/main
language: rust
---

# src

Oxide Mesh Net Daemon Binary

## Docstring

Oxide Mesh Net Daemon Binary

Production-grade background daemon coordinating multi-queue TUN interfaces,
dynamic MSS clamping, lock-free RCU Radix routing, DPLPMTUD probing, and secure IPC.

## Relationships

| Type | Target |
|------|--------|
| related | [Args](/crates/oxide-daemon/src/main/Args.md) |
| related | [main](/crates/oxide-daemon/src/main/main.md) |
| related | [clap](/_dependencies/cargo/clap.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
