---
okf_version: "0.2"
type: Function
title: update_acl
description: "[put(\"/acl\")]"
resource: crates/oxide-coordinator/src/handlers/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/acl/update_acl
language: rust
---

# update_acl

[put("/acl")]

## Signature

```rust
pub fn update_acl(
    data: web::Data<AppState>,
    req: web::Json<UpdateAclRequest>,
) -> Result<impl Responder>
```

## Decorators

- `put("/acl")`

## Visibility

- `pub`

## Docstring

[put("/acl")]

## Source
Lines 82–123 in `crates/oxide-coordinator/src/handlers/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-coordinator/src/handlers/acl.md) |
