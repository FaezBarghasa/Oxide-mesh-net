---
okf_version: "0.2"
type: Function
title: reenroll
description: "[post(\"/reenroll\")]"
resource: crates/oxide-coordinator/src/handlers/enrollment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/enrollment/reenroll
language: rust
---

# reenroll

[post("/reenroll")]

## Signature

```rust
pub fn reenroll(
    data: web::Data<AppState>,
    req: web::Json<ReenrollRequest>,
) -> Result<impl Responder>
```

## Decorators

- `post("/reenroll")`

## Visibility

- `pub`

## Docstring

[post("/reenroll")]

## Source
Lines 99–122 in `crates/oxide-coordinator/src/handlers/enrollment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [enrollment](/crates/oxide-coordinator/src/handlers/enrollment.md) |
