---
okf_version: "0.2"
type: Function
title: load_from_file
description: Load configuration from JSON file
resource: crates/oxide-daemon/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:46Z"
concept_id: crates/oxide-daemon/src/config/load_from_file_1
language: rust
---

# load_from_file

Load configuration from JSON file

## Signature

```rust
pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Load configuration from JSON file

## Source
Lines 50–54 in `crates/oxide-daemon/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-daemon/src/config.md) |
