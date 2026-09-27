---
okf_version: "0.2"
type: Module
title: socket
description: "Secure Local Daemon IPC Socket & Permissions Engine"
resource: crates/oxide-daemon/src/ipc/socket.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-daemon"
  - "git:branch:master"
  - "git:repo:Oxide-mesh-net"
timestamp: "2026-09-27T06:33:11Z"
concept_id: crates/oxide-daemon/src/ipc/socket
language: rust
---

# socket

Secure Local Daemon IPC Socket & Permissions Engine

## Docstring

Secure Local Daemon IPC Socket & Permissions Engine

Enforces OS-level access control on the local daemon IPC interface:
- POSIX: Unix Domain Socket (/var/run/oxide/daemon.sock) with 0660 permissions and SO_PEERCRED checks.
- Windows: Named Pipe (\\.\pipe\oxide-daemon) with strictly restricted DACL.

## Relationships

| Type | Target |
|------|--------|
| related | [PeerCredentials](/crates/oxide-daemon/src/ipc/socket/PeerCredentials.md) |
| related | [IpcHandler](/crates/oxide-daemon/src/ipc/socket/IpcHandler.md) |
| related | [SecureIpcServer](/crates/oxide-daemon/src/ipc/socket/SecureIpcServer.md) |
| related | [new](/crates/oxide-daemon/src/ipc/socket/new.md) |
| related | [bind](/crates/oxide-daemon/src/ipc/socket/bind.md) |
| related | [run_loop](/crates/oxide-daemon/src/ipc/socket/run_loop.md) |
| related | [handle_connection](/crates/oxide-daemon/src/ipc/socket/handle_connection.md) |
| related | [is_authorized_caller](/crates/oxide-daemon/src/ipc/socket/is_authorized_caller.md) |
| related | [socket_path](/crates/oxide-daemon/src/ipc/socket/socket_path.md) |
| related | [new](/crates/oxide-daemon/src/ipc/socket/new.md) |
| related | [bind](/crates/oxide-daemon/src/ipc/socket/bind.md) |
| related | [run_loop](/crates/oxide-daemon/src/ipc/socket/run_loop.md) |
| related | [handle_connection](/crates/oxide-daemon/src/ipc/socket/handle_connection.md) |
| related | [is_authorized_caller](/crates/oxide-daemon/src/ipc/socket/is_authorized_caller.md) |
| related | [socket_path](/crates/oxide-daemon/src/ipc/socket/socket_path.md) |
| related | [drop](/crates/oxide-daemon/src/ipc/socket/drop.md) |
| related | [drop](/crates/oxide-daemon/src/ipc/socket/drop.md) |
| related | [test_ipc_socket_bind_and_cleanup](/crates/oxide-daemon/src/ipc/socket/test_ipc_socket_bind_and_cleanup.md) |
| related | [test_caller_authorization](/crates/oxide-daemon/src/ipc/socket/test_caller_authorization.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
