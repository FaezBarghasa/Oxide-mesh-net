---
okf_version: "0.2"
type: Class
title: Cli
resource: crates/oxide-cli/src/main.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cli"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-cli/src/main/Cli
language: rust
---

# Cli

## Signature

```rust
struct Cli
```

## Decorators

- `derive(Parser, Debug)`
- `command(
    name = "oxide",
    about = "Management and diagnostic CLI for oxide-mesh-net",
    version
)`

## Methods

- `socket`
- `json`
- `command`

## Source
Lines 23–34 in `crates/oxide-cli/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/crates/oxide-cli/src/main.md) |
