# socket

## Classs

- [IpcHandler](IpcHandler.md) — Trait for handling IPC requests dispatched from socket server
- [PeerCredentials](PeerCredentials.md) — Caller credential metadata
- [SecureIpcServer](SecureIpcServer.md) — Secure Daemon IPC Server

## Functions

- [bind](bind.md) — Bind and configure Unix domain socket with secure 0660 file permissions
- [bind](bind_1.md) — Bind and configure Unix domain socket with secure 0660 file permissions
- [drop](drop.md)
- [drop](drop_1.md)
- [handle_connection](handle_connection.md) — Handle a single connection session
- [handle_connection](handle_connection_1.md) — Handle a single connection session
- [is_authorized_caller](is_authorized_caller.md) — Validate caller credentials
- [is_authorized_caller](is_authorized_caller_1.md) — Validate caller credentials
- [new](new.md)
- [new](new_1.md)
- [run_loop](run_loop.md) — Run the server loop, accepting incoming connections and dispatching to handler
- [run_loop](run_loop_1.md) — Run the server loop, accepting incoming connections and dispatching to handler
- [socket_path](socket_path.md)
- [socket_path](socket_path_1.md)
- [test_caller_authorization](test_caller_authorization.md) — [test]
- [test_ipc_socket_bind_and_cleanup](test_ipc_socket_bind_and_cleanup.md) — [tokio::test]
