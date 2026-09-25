//! SIMD evaluator for ACL rule acceleration

use crate::error::Result;
use crate::rules::{AclResult, AclRule, PacketMeta};

#[derive(Debug, Clone, Default)]
pub struct SimdEvaluator {
    #[allow(dead_code)]
    simd_available: bool,
}

impl SimdEvaluator {
    pub fn new() -> Result<Self> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
        let simd_available = true;
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
        let simd_available = false;

        Ok(Self { simd_available })
    }

    pub fn evaluate_batch(&self, _rules: &[AclRule], _meta: &PacketMeta) -> Option<AclResult> {
        None
    }
}