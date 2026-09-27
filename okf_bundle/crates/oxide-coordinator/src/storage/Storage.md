---
okf_version: "0.2"
type: Class
title: Storage
description: Main storage interface holding the configured backend
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage/Storage
language: rust
---

# Storage

Main storage interface holding the configured backend

## Signature

```rust
pub struct Storage
```

## Visibility

- `pub`

## Docstring

Main storage interface holding the configured backend

## Methods

- `backend`

## Source
Lines 59–61 in `crates/oxide-coordinator/src/storage.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [storage](/crates/oxide-coordinator/src/storage.md) |
| called_by | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| called_by | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| called_by | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| called_by | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| called_by | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| called_by | [init_schema](/crates/oxide-coordinator/src/storage/init_schema.md) |
| called_by | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| called_by | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| called_by | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| called_by | [new](/crates/oxide-coordinator/src/storage/new.md) |
| called_by | [new_async](/crates/oxide-coordinator/src/storage/new_async.md) |
| called_by | [new_mem](/crates/oxide-coordinator/src/storage/new_mem.md) |
| called_by | [new_surrealkv](/crates/oxide-coordinator/src/storage/new_surrealkv.md) |
| called_by | [new_ws](/crates/oxide-coordinator/src/storage/new_ws.md) |
| called_by | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| called_by | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| called_by | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| called_by | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| called_by | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
