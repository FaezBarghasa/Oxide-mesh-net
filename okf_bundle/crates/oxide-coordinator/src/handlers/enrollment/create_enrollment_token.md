---
okf_version: "0.2"
type: Function
title: create_enrollment_token
description: "[get(\"/enrollment-token\")]"
resource: crates/oxide-coordinator/src/handlers/enrollment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/enrollment/create_enrollment_token
language: rust
---

# create_enrollment_token

[get("/enrollment-token")]

## Signature

```rust
pub fn create_enrollment_token(
    data: web::Data<AppState>,
    query: web::Query<CreateTokenQuery>,
) -> Result<impl Responder>
```

## Decorators

- `get("/enrollment-token")`

## Visibility

- `pub`

## Docstring

[get("/enrollment-token")]

## Source
Lines 125–139 in `crates/oxide-coordinator/src/handlers/enrollment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [enrollment](/crates/oxide-coordinator/src/handlers/enrollment.md) |
