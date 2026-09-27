---
okf_version: "0.2"
type: Function
title: save_to_file
description: Save configuration to JSON file
resource: crates/oxide-daemon/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:46Z"
concept_id: crates/oxide-daemon/src/config/save_to_file
language: rust
---

# save_to_file

Save configuration to JSON file

## Signature

```rust
impl DaemonConfig { pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> }
```

## Visibility

- `pub`

## Docstring

Save configuration to JSON file

## Source
Lines 57–64 in `crates/oxide-daemon/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/oxide-daemon/src/config.md) |
