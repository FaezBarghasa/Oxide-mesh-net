---
okf_version: "0.2"
type: Function
title: step_down
description: Step down to follower
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster/step_down_1
language: rust
---

# step_down

Step down to follower

## Signature

```rust
pub fn step_down(&self, term: u64)
```

## Visibility

- `pub`

## Docstring

Step down to follower

## Source
Lines 151–162 in `crates/oxide-coordinator/src/cluster.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cluster](/crates/oxide-coordinator/src/cluster.md) |
