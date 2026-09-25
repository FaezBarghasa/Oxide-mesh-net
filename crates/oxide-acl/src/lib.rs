//! SIMD-accelerated ACL and microsegmentation engine for oxide-mesh-net

pub mod error;
pub mod rules;
pub mod simd;

pub use error::{AclError, Result};
pub use rules::*;
pub use simd::*;