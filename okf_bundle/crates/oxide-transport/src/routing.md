---
okf_version: "0.2"
type: Module
title: routing
description: Lock-free RCU prefix routing table and L1 direct-mapped routing cache
resource: crates/oxide-transport/src/routing.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-transport"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:27:12Z"
concept_id: crates/oxide-transport/src/routing
language: rust
---

# routing

Lock-free RCU prefix routing table and L1 direct-mapped routing cache

## Docstring

Lock-free RCU prefix routing table and L1 direct-mapped routing cache

Provides ultra-high-throughput routing lookups (<2ns L1 cache hit, lock-free ArcSwap
Radix tree lookup on miss) with zero reader-writer lock contention across worker cores.

## Relationships

| Type | Target |
|------|--------|
| related | [RouteTarget](/crates/oxide-transport/src/routing/RouteTarget.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [with_pmtu](/crates/oxide-transport/src/routing/with_pmtu.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [with_pmtu](/crates/oxide-transport/src/routing/with_pmtu.md) |
| related | [RouteEntry](/crates/oxide-transport/src/routing/RouteEntry.md) |
| related | [RadixRoutingTable](/crates/oxide-transport/src/routing/RadixRoutingTable.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [insert](/crates/oxide-transport/src/routing/insert.md) |
| related | [remove](/crates/oxide-transport/src/routing/remove.md) |
| related | [all_routes](/crates/oxide-transport/src/routing/all_routes.md) |
| related | [lookup](/crates/oxide-transport/src/routing/lookup.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [insert](/crates/oxide-transport/src/routing/insert.md) |
| related | [remove](/crates/oxide-transport/src/routing/remove.md) |
| related | [all_routes](/crates/oxide-transport/src/routing/all_routes.md) |
| related | [lookup](/crates/oxide-transport/src/routing/lookup.md) |
| related | [L1DirectMappedCache](/crates/oxide-transport/src/routing/L1DirectMappedCache.md) |
| related | [default](/crates/oxide-transport/src/routing/default.md) |
| related | [default](/crates/oxide-transport/src/routing/default.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [hash_index](/crates/oxide-transport/src/routing/hash_index.md) |
| related | [get](/crates/oxide-transport/src/routing/get.md) |
| related | [insert](/crates/oxide-transport/src/routing/insert.md) |
| related | [invalidate](/crates/oxide-transport/src/routing/invalidate.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [hash_index](/crates/oxide-transport/src/routing/hash_index.md) |
| related | [get](/crates/oxide-transport/src/routing/get.md) |
| related | [insert](/crates/oxide-transport/src/routing/insert.md) |
| related | [invalidate](/crates/oxide-transport/src/routing/invalidate.md) |
| related | [RcuRouter](/crates/oxide-transport/src/routing/RcuRouter.md) |
| related | [default](/crates/oxide-transport/src/routing/default.md) |
| related | [default](/crates/oxide-transport/src/routing/default.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [from_table](/crates/oxide-transport/src/routing/from_table.md) |
| related | [lookup](/crates/oxide-transport/src/routing/lookup.md) |
| related | [load_table](/crates/oxide-transport/src/routing/load_table.md) |
| related | [update](/crates/oxide-transport/src/routing/update.md) |
| related | [new](/crates/oxide-transport/src/routing/new.md) |
| related | [from_table](/crates/oxide-transport/src/routing/from_table.md) |
| related | [lookup](/crates/oxide-transport/src/routing/lookup.md) |
| related | [load_table](/crates/oxide-transport/src/routing/load_table.md) |
| related | [update](/crates/oxide-transport/src/routing/update.md) |
| related | [test_radix_routing_longest_prefix_match](/crates/oxide-transport/src/routing/test_radix_routing_longest_prefix_match.md) |
| related | [test_rcu_router_with_l1_cache](/crates/oxide-transport/src/routing/test_rcu_router_with_l1_cache.md) |
