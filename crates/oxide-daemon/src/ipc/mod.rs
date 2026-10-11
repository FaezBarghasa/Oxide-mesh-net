//! IPC communication module for oxide-daemon

pub mod client;
pub mod protocol;
pub mod socket;

pub use client::IpcClient;
pub use protocol::{
    DaemonStatusDto, IpcRequest, IpcResponse, MAX_IPC_FRAME_SIZE, NetcheckDto, PeerStatusDto,
    RouteEntryDto, VhostRouteDto, read_frame, write_frame,
};
pub use socket::{
    DEFAULT_SOCKET_PATH, FALLBACK_SOCKET_PATH, IpcHandler, PeerCredentials, SecureIpcServer,
};
