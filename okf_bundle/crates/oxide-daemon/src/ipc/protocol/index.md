# protocol

## Classs

- [DaemonStatusDto](DaemonStatusDto.md) — High-level daemon status DTO
- [IpcRequest](IpcRequest.md) — Request messages sent from CLI to Daemon
- [IpcResponse](IpcResponse.md) — Response messages returned from Daemon to CLI
- [PeerStatusDto](PeerStatusDto.md) — Peer connection state DTO
- [RouteEntryDto](RouteEntryDto.md) — Routing entry DTO

## Functions

- [read_frame](read_frame.md) — Read a length-prefixed JSON frame asynchronously
- [test_ipc_framing_roundtrip](test_ipc_framing_roundtrip.md) — [tokio::test]
- [write_frame](write_frame.md) — Write a length-prefixed JSON frame asynchronously
