---
okf_version: "0.2"
type: Function
title: apply_entry
description: Apply an entry to the replicated state machine
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/apply_entry
language: rust
---

# apply_entry

Apply an entry to the replicated state machine

## Signature

```rust
impl ClusterEngine { pub fn apply_entry(&self, entry: ClusterLogEntry) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Apply an entry to the replicated state machine

## Source
Lines 103–131 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
