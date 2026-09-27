---
okf_version: "0.2"
type: Function
title: confirmed_pmtu
description: Return the currently confirmed active Path MTU
resource: crates/oxide-transport/src/dplpmtud.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:14:57Z"
concept_id: crates/oxide-transport/src/dplpmtud/confirmed_pmtu
language: rust
---

# confirmed_pmtu

Return the currently confirmed active Path MTU

## Signature

```rust
impl DplpmtudEngine { pub fn confirmed_pmtu(&self) -> u16 }
```

## Visibility

- `pub`

## Docstring

Return the currently confirmed active Path MTU
[inline]

## Source
Lines 76–78 in `crates/oxide-transport/src/dplpmtud.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dplpmtud](/crates/oxide-transport/src/dplpmtud.md) |
