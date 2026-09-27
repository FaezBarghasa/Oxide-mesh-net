---
okf_version: "0.2"
type: Function
title: enroll
description: "[post(\"/enroll\")]"
resource: crates/oxide-coordinator/src/handlers/enrollment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/enrollment/enroll
language: rust
---

# enroll

[post("/enroll")]

## Signature

```rust
pub fn enroll(
    data: web::Data<AppState>,
    req: web::Json<EnrollRequest>,
) -> Result<impl Responder>
```

## Decorators

- `post("/enroll")`

## Visibility

- `pub`

## Docstring

[post("/enroll")]

## Source
Lines 40–90 in `crates/oxide-coordinator/src/handlers/enrollment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [enrollment](/crates/oxide-coordinator/src/handlers/enrollment.md) |
