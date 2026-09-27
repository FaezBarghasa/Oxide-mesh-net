---
okf_version: "0.2"
type: Module
title: src
description: Oxide Mesh Net CLI Tool
resource: crates/oxide-cli/src/main.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-cli"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-cli/src/main
language: rust
---

# src

Oxide Mesh Net CLI Tool

## Docstring

Oxide Mesh Net CLI Tool

Provides operational management and diagnostic controls for oxide-mesh-net:
- `oxide status` : Inspects daemon health, PMTU, circuit breaker state, and port hopping.
- `oxide up`     : Brings interface online.
- `oxide down`   : Tears down interface.
- `oxide routes` : Displays active lock-free RCU Radix routing table.
- `oxide peers`  : Displays mesh peer connection statistics.
- `oxide acl`    : Dynamically inspects or reloads SIMD ACL policies.
- `oxide ping`   : Verifies daemon IPC latency and responsiveness.

## Relationships

| Type | Target |
|------|--------|
| related | [Cli](/crates/oxide-cli/src/main/Cli.md) |
| related | [Commands](/crates/oxide-cli/src/main/Commands.md) |
| related | [AclCommand](/crates/oxide-cli/src/main/AclCommand.md) |
| related | [AclSubcommands](/crates/oxide-cli/src/main/AclSubcommands.md) |
| related | [main](/crates/oxide-cli/src/main/main.md) |
| related | [clap](/_dependencies/cargo/clap.md) |
| related | [instant](/_dependencies/cargo/instant.md) |
