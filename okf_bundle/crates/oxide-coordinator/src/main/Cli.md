---
okf_version: "0.2"
type: Class
title: Cli
description: "[derive(Parser, Debug)]"
resource: crates/oxide-coordinator/src/main.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T12:10:51Z"
concept_id: crates/oxide-coordinator/src/main/Cli
language: rust
---

# Cli

[derive(Parser, Debug)]

## Signature

```rust
struct Cli
```

## Decorators

- `derive(Parser, Debug)`
- `command(name = "oxide-coordinator")`
- `command(author = "Faez Barghasa <faez.barghasa@gmail.com>")`
- `command(version = "0.1.0")`
- `command(about = "Embedded MQTT broker and Actix Web coordinator for oxide-mesh-net", long_about = None)`

## Docstring

[derive(Parser, Debug)]
[command(name = "oxide-coordinator")]
[command(author = "Faez Barghasa <faez.barghasa@gmail.com>")]
[command(version = "0.1.0")]
[command(about = "Embedded MQTT broker and Actix Web coordinator for oxide-mesh-net", long_about = None)]

## Methods

- `mesh`
- `bind`
- `storage`
- `namespace`
- `database`

## Source
Lines 16–36 in `crates/oxide-coordinator/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/crates/oxide-coordinator/src/main.md) |
