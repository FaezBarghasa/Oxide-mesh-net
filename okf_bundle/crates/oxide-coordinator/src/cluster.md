---
okf_version: "0.2"
type: Module
title: cluster
description: "Replicated Cluster State Machine & Consensus Engine for Multi-Coordinator Deployments"
resource: crates/oxide-coordinator/src/cluster.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-coordinator"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:16:26Z"
concept_id: crates/oxide-coordinator/src/cluster
language: rust
---

# cluster

Replicated Cluster State Machine & Consensus Engine for Multi-Coordinator Deployments

## Docstring

Replicated Cluster State Machine & Consensus Engine for Multi-Coordinator Deployments

Replicates node registration, routing matrices, ACL policies, and RFC 8628 device grants
across coordinator nodes to eliminate Single Points of Failure (SPOF) and provide <150ms leader failover.

## Relationships

| Type | Target |
|------|--------|
| related | [ClusterRole](/crates/oxide-coordinator/src/cluster/ClusterRole.md) |
| related | [ClusterLogEntry](/crates/oxide-coordinator/src/cluster/ClusterLogEntry.md) |
| related | [ClusterConfig](/crates/oxide-coordinator/src/cluster/ClusterConfig.md) |
| related | [default](/crates/oxide-coordinator/src/cluster/default.md) |
| related | [default](/crates/oxide-coordinator/src/cluster/default.md) |
| related | [ReplicatedState](/crates/oxide-coordinator/src/cluster/ReplicatedState.md) |
| related | [ClusterEngine](/crates/oxide-coordinator/src/cluster/ClusterEngine.md) |
| related | [new](/crates/oxide-coordinator/src/cluster/new.md) |
| related | [role](/crates/oxide-coordinator/src/cluster/role.md) |
| related | [is_leader](/crates/oxide-coordinator/src/cluster/is_leader.md) |
| related | [apply_entry](/crates/oxide-coordinator/src/cluster/apply_entry.md) |
| related | [is_device_grant_authorized](/crates/oxide-coordinator/src/cluster/is_device_grant_authorized.md) |
| related | [promote_to_leader](/crates/oxide-coordinator/src/cluster/promote_to_leader.md) |
| related | [step_down](/crates/oxide-coordinator/src/cluster/step_down.md) |
| related | [new](/crates/oxide-coordinator/src/cluster/new.md) |
| related | [role](/crates/oxide-coordinator/src/cluster/role.md) |
| related | [is_leader](/crates/oxide-coordinator/src/cluster/is_leader.md) |
| related | [apply_entry](/crates/oxide-coordinator/src/cluster/apply_entry.md) |
| related | [is_device_grant_authorized](/crates/oxide-coordinator/src/cluster/is_device_grant_authorized.md) |
| related | [promote_to_leader](/crates/oxide-coordinator/src/cluster/promote_to_leader.md) |
| related | [step_down](/crates/oxide-coordinator/src/cluster/step_down.md) |
| related | [test_cluster_state_replication](/crates/oxide-coordinator/src/cluster/test_cluster_state_replication.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
