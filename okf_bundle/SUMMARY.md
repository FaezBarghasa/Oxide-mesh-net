---
description: 'Top-level OKF summary: 2795 concepts across 1 domains and 102 modules'
git_branch: master
git_repo: Oxide-mesh-net
okf_version: '0.2'
timestamp: '2026-09-27T20:27:49Z'
title: Oxide-mesh-net — Knowledge Summary
type: Index
---

# Oxide-mesh-net — Knowledge Summary

> OKF v0.2 bundle | 2,795 concepts | 1 domains | 102 modules

## Stats

| Type | Count |
|------|-------|
| Dependency | 1,402 |
| Function | 1,047 |
| Class | 244 |
| Module | 102 |

| Language | Concepts |
|----------|----------|
| manifest | 1,402 |
| rust | 1,393 |

## Domain Map

Use these links to navigate the bundle or prime an AI agent with focused context.

### [crates](crates/index.md) — 1,393 concepts

- [crates/oxide-coordinator/src/storage](crates/oxide-coordinator/src/storage/index.md) (125 concepts) — Production Storage Layer for Oxide Coordinator using SurrealDB 3.3.0
- [crates/oxide-crypto/src/keys](crates/oxide-crypto/src/keys/index.md) (102 concepts) — Cryptographic key types and identity management
- [crates/oxide-protocol/src/topics](crates/oxide-protocol/src/topics/index.md) (101 concepts)
- [crates/oxide-core/src/types](crates/oxide-core/src/types/index.md) (76 concepts) — Core types for oxide-mesh-net
- [crates/oxide-protocol/src/wire](crates/oxide-protocol/src/wire/index.md) (56 concepts) — Wire protocol framing for data plane packets
- [crates/oxide-tun/src/platform/linux](crates/oxide-tun/src/platform/linux/index.md) (55 concepts) — Linux TUN/TAP implementation with multi-queue support
- [crates/oxide-acl/src/rules](crates/oxide-acl/src/rules/index.md) (52 concepts) — ACL rule definitions and SIMD-accelerated evaluation
- [crates/oxide-tun/src/platform/macos](crates/oxide-tun/src/platform/macos/index.md) (47 concepts) — macOS utun implementation
- *…and 94 more modules*

## Dependencies

> Full list at [`_dependencies/index.md`](/_dependencies/index.md) or `okf lookup --type Dependency`

| Ecosystem | Packages |
|----------|----------|
| cargo | 1,402 |

## Key Concepts

Highest-value concepts across all domains (Classes and Functions with rich descriptions).

| Concept | Type | Module | Description |
|---------|------|--------|-------------|
| [pull_chunk](/crates/oxide-daemon/src/services/pty_backpressure/pull_chunk.md) | Function | `crates/oxide-daemon/src` | Pull bytes for transmission over QUIC stream, releasing back… |
| [pull_chunk](/crates/oxide-daemon/src/services/pty_backpressure/pull_chunk_1.md) | Function | `crates/oxide-daemon/src` | Pull bytes for transmission over QUIC stream, releasing back… |
| [is_valid_port](/crates/oxide-transport/src/porthopper/is_valid_port.md) | Function | `crates/oxide-transport/src` | Return true if a received packet on `dest_port` belongs to t… |
| [is_valid_port](/crates/oxide-transport/src/porthopper/is_valid_port_1.md) | Function | `crates/oxide-transport/src` | Return true if a received packet on `dest_port` belongs to t… |
| [active_ports](/crates/oxide-transport/src/porthopper/active_ports.md) | Function | `crates/oxide-transport/src` | Return active listening port triple [P_prev, P_current, P_ne… |
| [active_ports](/crates/oxide-transport/src/porthopper/active_ports_1.md) | Function | `crates/oxide-transport/src` | Return active listening port triple [P_prev, P_current, P_ne… |
| [new](/crates/oxide-coordinator/src/storage/new.md) | Function | `crates/oxide-coordinator/src` | Synchronous initialization wrapper (defaults to in-memory or… |
| [new](/crates/oxide-coordinator/src/storage/new_1.md) | Function | `crates/oxide-coordinator/src` | Synchronous initialization wrapper (defaults to in-memory or… |
| [AclAction](/crates/oxide-protocol/src/wire/AclAction.md) | Class | `crates/oxide-protocol/src` | [derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize,… |
| [AclDirection](/crates/oxide-protocol/src/wire/AclDirection.md) | Class | `crates/oxide-protocol/src` | [derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize,… |
| [run_loop](/crates/oxide-daemon/src/ipc/socket/run_loop.md) | Function | `crates/oxide-daemon/src` | Run the server loop, accepting incoming connections and disp… |
| [run_loop](/crates/oxide-daemon/src/ipc/socket/run_loop_1.md) | Function | `crates/oxide-daemon/src` | Run the server loop, accepting incoming connections and disp… |
| [OverlayIp](/crates/oxide-core/src/types/OverlayIp.md) | Class | `crates/oxide-core/src` | Overlay IP address assignment (IPv4 in 100.64.0.0/10 CGNAT r… |
| [insert](/crates/oxide-transport/src/routing/insert.md) | Function | `crates/oxide-transport/src` | Insert or replace a route entry and sort by longest prefix l… |
| [insert](/crates/oxide-transport/src/routing/insert_1.md) | Function | `crates/oxide-transport/src` | Insert or replace a route entry and sort by longest prefix l… |
| [MemoryStorage](/crates/oxide-coordinator/src/storage/MemoryStorage.md) | Class | `crates/oxide-coordinator/src` | ============================================================… |
| [lwt](/crates/oxide-protocol/src/topics/lwt.md) | Function | `crates/oxide-protocol/src` | Last Will Testament (set on connect, published on ungraceful… |
| [lwt](/crates/oxide-protocol/src/topics/lwt_1.md) | Function | `crates/oxide-protocol/src` | Last Will Testament (set on connect, published on ungraceful… |
| [create_tun_windows](/crates/oxide-tun/src/platform/windows/create_tun_windows.md) | Function | `crates/oxide-tun/src` | AsyncRead/AsyncWrite would be implemented using Wintun's rin… |
| [bind](/crates/oxide-daemon/src/ipc/socket/bind.md) | Function | `crates/oxide-daemon/src` | Bind and configure Unix domain socket with secure 0660 file … |

## Usage with OpenCode

```bash
# Prime full context
RUN cat ./okf_bundle/SUMMARY.md

# Prime specific domain
RUN cat ./okf_bundle/crates/index.md

# Find a concept
RUN find ./okf_bundle -name '<ConceptName>.md' | xargs cat
```
