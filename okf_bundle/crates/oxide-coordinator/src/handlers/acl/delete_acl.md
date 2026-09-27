---
okf_version: "0.2"
type: Function
title: delete_acl
description: "[delete(\"/acl\")]"
resource: crates/oxide-coordinator/src/handlers/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/handlers/acl/delete_acl
language: rust
---

# delete_acl

[delete("/acl")]

## Signature

```rust
pub fn delete_acl(data: web::Data<AppState>) -> Result<impl Responder>
```

## Decorators

- `delete("/acl")`

## Visibility

- `pub`

## Docstring

[delete("/acl")]

## Source
Lines 126–136 in `crates/oxide-coordinator/src/handlers/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-coordinator/src/handlers/acl.md) |
