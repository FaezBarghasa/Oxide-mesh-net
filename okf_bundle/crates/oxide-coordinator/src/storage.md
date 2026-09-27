---
okf_version: "0.2"
type: Module
title: storage
description: Production Storage Layer for Oxide Coordinator using SurrealDB 3.3.0
resource: crates/oxide-coordinator/src/storage.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/storage
language: rust
---

# storage

Production Storage Layer for Oxide Coordinator using SurrealDB 3.3.0

## Docstring

Production Storage Layer for Oxide Coordinator using SurrealDB 3.3.0

Multi-Model & Graph Architecture:
- **SurrealDB 3.3.0**: Authoritative persistence, graph relations (mesh topology,
shortest path, peer links), document records, and live change-feed queries.
- In-Memory (`Mem`), Embedded Disk (`SurrealKv`), or Distributed Remote (`Ws`).

## Relationships

| Type | Target |
|------|--------|
| related | [StorageBackend](/crates/oxide-coordinator/src/storage/StorageBackend.md) |
| related | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| related | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| related | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| related | [Storage](/crates/oxide-coordinator/src/storage/Storage.md) |
| related | [new_async](/crates/oxide-coordinator/src/storage/new_async.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| related | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| related | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| related | [new_async](/crates/oxide-coordinator/src/storage/new_async.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| related | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| related | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| related | [SurrealEngine](/crates/oxide-coordinator/src/storage/SurrealEngine.md) |
| related | [SurrealStorage](/crates/oxide-coordinator/src/storage/SurrealStorage.md) |
| related | [SurrealNodeRecord](/crates/oxide-coordinator/src/storage/SurrealNodeRecord.md) |
| related | [SurrealRouteRecord](/crates/oxide-coordinator/src/storage/SurrealRouteRecord.md) |
| related | [SurrealAclRecord](/crates/oxide-coordinator/src/storage/SurrealAclRecord.md) |
| related | [SurrealPresenceRecord](/crates/oxide-coordinator/src/storage/SurrealPresenceRecord.md) |
| related | [init_schema](/crates/oxide-coordinator/src/storage/init_schema.md) |
| related | [new_mem](/crates/oxide-coordinator/src/storage/new_mem.md) |
| related | [new_surrealkv](/crates/oxide-coordinator/src/storage/new_surrealkv.md) |
| related | [new_ws](/crates/oxide-coordinator/src/storage/new_ws.md) |
| related | [init_schema](/crates/oxide-coordinator/src/storage/init_schema.md) |
| related | [new_mem](/crates/oxide-coordinator/src/storage/new_mem.md) |
| related | [new_surrealkv](/crates/oxide-coordinator/src/storage/new_surrealkv.md) |
| related | [new_ws](/crates/oxide-coordinator/src/storage/new_ws.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| related | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| related | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [record_presence](/crates/oxide-coordinator/src/storage/record_presence.md) |
| related | [is_node_online](/crates/oxide-coordinator/src/storage/is_node_online.md) |
| related | [record_topology_link](/crates/oxide-coordinator/src/storage/record_topology_link.md) |
| related | [MemoryStorage](/crates/oxide-coordinator/src/storage/MemoryStorage.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [SledStorage](/crates/oxide-coordinator/src/storage/SledStorage.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [new](/crates/oxide-coordinator/src/storage/new.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [get_node_metadata](/crates/oxide-coordinator/src/storage/get_node_metadata.md) |
| related | [set_node_metadata](/crates/oxide-coordinator/src/storage/set_node_metadata.md) |
| related | [delete_node_metadata](/crates/oxide-coordinator/src/storage/delete_node_metadata.md) |
| related | [list_nodes](/crates/oxide-coordinator/src/storage/list_nodes.md) |
| related | [get_route](/crates/oxide-coordinator/src/storage/get_route.md) |
| related | [set_route](/crates/oxide-coordinator/src/storage/set_route.md) |
| related | [delete_route](/crates/oxide-coordinator/src/storage/delete_route.md) |
| related | [list_routes](/crates/oxide-coordinator/src/storage/list_routes.md) |
| related | [get_acl_policy](/crates/oxide-coordinator/src/storage/get_acl_policy.md) |
| related | [set_acl_policy](/crates/oxide-coordinator/src/storage/set_acl_policy.md) |
| related | [test_surreal_storage_nodes](/crates/oxide-coordinator/src/storage/test_surreal_storage_nodes.md) |
| related | [test_surreal_storage_routes_and_topology](/crates/oxide-coordinator/src/storage/test_surreal_storage_routes_and_topology.md) |
| related | [test_memory_storage_fallback](/crates/oxide-coordinator/src/storage/test_memory_storage_fallback.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
| related | [surrealdb](/_dependencies/cargo/surrealdb.md) |
| related | [sled](/_dependencies/cargo/sled.md) |
