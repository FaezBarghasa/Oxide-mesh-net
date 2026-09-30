//! NAT traversal, STUN discovery, UPnP port mapping, and Magicsock direct P2P coordinator

pub mod magicsock;
pub mod stun;
pub mod upnp;

pub use magicsock::{EndpointCandidate, MagicsockEngine, PathType};
pub use stun::{NatType, StunClient};
pub use upnp::{UpnpClient, UpnpConfig};
