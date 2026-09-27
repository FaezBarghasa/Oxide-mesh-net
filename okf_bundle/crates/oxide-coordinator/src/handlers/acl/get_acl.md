---
okf_version: "0.2"
type: Function
title: get_acl
description: "[get(\"/acl\")]"
resource: crates/oxide-coordinator/src/handlers/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/acl/get_acl
language: rust
---

# get_acl

[get("/acl")]

## Signature

```rust
pub fn get_acl(data: web::Data<AppState>) -> Result<impl Responder>
```

## Decorators

- `get("/acl")`

## Visibility

- `pub`

## Docstring

[get("/acl")]

## Source
Lines 54–79 in `crates/oxide-coordinator/src/handlers/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-coordinator/src/handlers/acl.md) |
