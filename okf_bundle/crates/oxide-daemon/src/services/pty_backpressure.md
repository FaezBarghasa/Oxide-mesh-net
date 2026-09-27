---
okf_version: "0.2"
type: Module
title: pty_backpressure
description: "Oxide-SSH Pseudo-Terminal (PTY) Backpressure & Bounded Ring Buffer"
resource: crates/oxide-daemon/src/services/pty_backpressure.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/services/pty_backpressure
language: rust
---

# pty_backpressure

Oxide-SSH Pseudo-Terminal (PTY) Backpressure & Bounded Ring Buffer

## Docstring

Oxide-SSH Pseudo-Terminal (PTY) Backpressure & Bounded Ring Buffer

Enforces a strict 256 KB memory bound on PTY terminal buffers, suspending reads
when the remote stream buffer is saturated to prevent runaway memory exhaustion.

## Relationships

| Type | Target |
|------|--------|
| related | [PtyStreamController](/crates/oxide-daemon/src/services/pty_backpressure/PtyStreamController.md) |
| related | [default](/crates/oxide-daemon/src/services/pty_backpressure/default.md) |
| related | [default](/crates/oxide-daemon/src/services/pty_backpressure/default.md) |
| related | [new](/crates/oxide-daemon/src/services/pty_backpressure/new.md) |
| related | [push_chunk](/crates/oxide-daemon/src/services/pty_backpressure/push_chunk.md) |
| related | [pull_chunk](/crates/oxide-daemon/src/services/pty_backpressure/pull_chunk.md) |
| related | [is_suspended](/crates/oxide-daemon/src/services/pty_backpressure/is_suspended.md) |
| related | [buffered_bytes](/crates/oxide-daemon/src/services/pty_backpressure/buffered_bytes.md) |
| related | [new](/crates/oxide-daemon/src/services/pty_backpressure/new.md) |
| related | [push_chunk](/crates/oxide-daemon/src/services/pty_backpressure/push_chunk.md) |
| related | [pull_chunk](/crates/oxide-daemon/src/services/pty_backpressure/pull_chunk.md) |
| related | [is_suspended](/crates/oxide-daemon/src/services/pty_backpressure/is_suspended.md) |
| related | [buffered_bytes](/crates/oxide-daemon/src/services/pty_backpressure/buffered_bytes.md) |
| related | [test_pty_backpressure_suspension_and_resume](/crates/oxide-daemon/src/services/pty_backpressure/test_pty_backpressure_suspension_and_resume.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
