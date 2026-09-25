//! SIMD-accelerated ACL and microsegmentation engine

pub mod error;
pub mod rules;
pub mod simd;

pub use error::{AclError, Result};
pub use rules::{AclEngine, AclRule, AclAction, AclDirection, Protocol, PortRange, PacketMeta, AclResult, CompiledRule};
pub use simd::{SimdEvaluator, port_bitmap, protocol_bitmap};