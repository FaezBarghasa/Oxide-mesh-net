---
okf_version: "0.2"
type: Function
title: health_check
resource: crates/oxide-coordinator/src/handlers/health.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/health/health_check
language: rust
---

# health_check

## Signature

```rust
pub fn health_check(data: web::Data<AppState>) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 27–33 in `crates/oxide-coordinator/src/handlers/health.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [health](/crates/oxide-coordinator/src/handlers/health.md) |
