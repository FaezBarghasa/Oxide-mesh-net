---
okf_version: "0.2"
type: Module
title: src
description: oxide-coordinator standalone daemon binary
resource: crates/oxide-coordinator/src/main.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T12:10:51Z"
concept_id: crates/oxide-coordinator/src/main
language: rust
---

# src

oxide-coordinator standalone daemon binary

## Docstring

oxide-coordinator standalone daemon binary

Embedded MQTT broker, SurrealDB 3 multi-model persistence, and Actix Web REST/WS coordinator.

## Relationships

| Type | Target |
|------|--------|
| related | [Cli](/crates/oxide-coordinator/src/main/Cli.md) |
| related | [main](/crates/oxide-coordinator/src/main/main.md) |
| related | [clap](/_dependencies/cargo/clap.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
