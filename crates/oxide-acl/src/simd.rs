//! SIMD-accelerated ACL evaluation

use std::simd::{u64x4, u32x4, u16x8, SimdPartialEq, SimdPartialOrd, Mask};
use crate::rules::{AclRule, PacketMeta, CompiledRule, AclResult, PrefixTrie};
use crate::error::{AclError, Result};

/// SIMD evaluator for batch packet processing
pub struct SimdEvaluator {
    has_avx2: bool,
    has_avx512: bool,
    has_neon: bool,
}

impl SimdEvaluator {
    pub fn new() -> Result<Self> {
        // Detect CPU features at runtime
        #[cfg(target_arch = "x86_64")]
        let (has_avx2, has_avx512) = {
            use std::arch::is_x86_feature_detected;
            (is_x86_feature_detected!("avx2"), is_x86_feature_detected!("avx512f"))
        };
        #[cfg(target_arch = "aarch64")]
        let (has_avx2, has_avx512) = (false, false);
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let (has_avx2, has_avx512) = (false, false);

        // NEON detection for ARM
        #[cfg(target_arch = "aarch64")]
        let has_neon = true;
        #[cfg(not(target_arch = "aarch64"))]
        let has_neon = false;

        Ok(Self {
            has_avx2,
            has_avx512,
            has_neon,
        })
    }

    /// Evaluate a batch of packets using SIMD
    pub fn evaluate_batch(&self, rules: &[AclRule], meta: &PacketMeta) -> Option<AclResult> {
        if !self.has_simd() {
            return None;
        }

        // For single packet, SIMD doesn't help much
        // This would be used for batch evaluation
        None
    }

    /// Check if SIMD is available
    pub fn has_simd(&self) -> bool {
        self.has_avx2 || self.has_avx512 || self.has_neon
    }

    /// Evaluate multiple packets at once (for batch processing)
    pub fn evaluate_batch_simd(&self, rules: &[AclRule], metas: &[PacketMeta]) -> Vec<AclResult> {
        if !self.has_simd() || metas.len() < 4 {
            // Fall back to scalar
            return metas.iter().map(|m| {
                rules.iter()
                    .find(|r| self.match_rule_scalar(r, m))
                    .map(|r| match r.action {
                        crate::rules::AclAction::Allow => AclResult::Allow,
                        crate::rules::AclAction::Deny => AclResult::Deny,
                        crate::rules::AclAction::Log => AclResult::Log,
                    })
                    .unwrap_or(AclResult::Allow)
            }).collect();
        }

        // SIMD batch evaluation would go here
        // For now, return scalar results
        metas.iter().map(|m| {
            rules.iter()
                .find(|r| self.match_rule_scalar(r, m))
                .map(|r| match r.action {
                    crate::rules::AclAction::Allow => AclResult::Allow,
                    crate::rules::AclAction::Deny => AclResult::Deny,
                    crate::rules::AclAction::Log => AclResult::Log,
                })
                .unwrap_or(AclResult::Allow)
        }).collect()
    }

    fn match_rule_scalar(&self, rule: &AclRule, meta: &PacketMeta) -> bool {
        // Direction check
        if rule.direction != crate::rules::AclDirection::Both && rule.direction != meta.direction {
            return false;
        }

        // Protocol check (SIMD-friendly bitmap)
        if let Some(compiled) = &rule.compiled {
            let proto_bit = 1u32 << (meta.protocol as u8);
            if compiled.protocol_bitmap & proto_bit == 0 && !rule.protocols.is_empty() {
                return false;
            }
        } else if !rule.protocols.is_empty() && !rule.protocols.iter().any(|p| *p == meta.protocol) {
            return false;
        }

        // Port checks using bitmaps (SIMD-friendly)
        if let Some(compiled) = &rule.compiled {
            if let Some(src_port_bitmap) = &compiled.src_port_bitmap {
                let idx = meta.src_port as usize / 64;
                let bit = meta.src_port as usize % 64;
                if idx < src_port_bitmap.len() && (src_port_bitmap[idx] & (1u64 << bit)) == 0 {
                    return false;
                }
            }
            if let Some(dst_port_bitmap) = &compiled.dst_port_bitmap {
                let idx = meta.dst_port as usize / 64;
                let bit = meta.dst_port as usize % 64;
                if idx < dst_port_bitmap.len() && (dst_port_bitmap[idx] & (1u64 << bit)) == 0 {
                    return false;
                }
            }
        }

        // Prefix checks (trie-based, not SIMD-friendly but fast)
        if let Some(compiled) = &rule.compiled {
            if let Some(trie) = &compiled.src_prefix_trie {
                if trie.match_ip(meta.src_ip).is_empty() {
                    return false;
                }
            }
            if let Some(trie) = &compiled.dst_prefix_trie {
                if trie.match_ip(meta.dst_ip).is_empty() {
                    return false;
                }
            }
        }

        // Identity check
        if !rule.src_identities.is_empty() {
            if let Some(identity) = meta.src_identity {
                if !rule.src_identities.iter().any(|id| *id == identity) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// SIMD-optimized port bitmap operations
pub mod port_bitmap {
    use std::simd::{u64x4, Mask};

    /// Check if port is in bitmap using SIMD
    #[inline]
    pub fn contains_port_simd(bitmap: &[u64], port: u16) -> bool {
        let idx = port as usize / 64;
        let bit = port as usize % 64;
        if idx >= bitmap.len() {
            return false;
        }
        (bitmap[idx] & (1u64 << bit)) != 0
    }

    /// Check multiple ports at once using SIMD
    #[inline]
    pub fn contains_ports_simd(bitmap: &[u64], ports: &[u16]) -> Vec<bool> {
        // Process 4 ports at a time with u64x4
        let mut results = Vec::with_capacity(ports.len());
        
        for chunk in ports.chunks(4) {
            if chunk.len() == 4 {
                let idxs = u64x4::from_array([
                    (chunk[0] as usize / 64) as u64,
                    (chunk[1] as usize / 64) as u64,
                    (chunk[2] as usize / 64) as u64,
                    (chunk[3] as usize / 64) as u64,
                ]);
                let bits = u64x4::from_array([
                    1u64 << (chunk[0] as usize % 64),
                    1u64 << (chunk[1] as usize % 64),
                    1u64 << (chunk[2] as usize % 64),
                    1u64 << (chunk[3] as usize % 64),
                ]);

                // Load bitmap values (would need gather, not available in stable SIMD)
                // For now, scalar fallback
                for &port in chunk {
                    results.push(contains_port_simd(bitmap, port));
                }
            } else {
                for &port in chunk {
                    results.push(contains_port_simd(bitmap, port));
                }
            }
        }
        results
    }

    /// Build port bitmap from ranges
    pub fn build_bitmap(ranges: &[(u16, u16)]) -> Vec<u64> {
        let mut bitmap = vec![0u64; 1024]; // 65536 bits
        for &(start, end) in ranges {
            for port in start..=end {
                let idx = port as usize / 64;
                let bit = port as usize % 64;
                bitmap[idx] |= 1u64 << bit;
            }
        }
        bitmap
    }
}

/// SIMD-optimized protocol bitmap operations
pub mod protocol_bitmap {
    use std::simd::u32x4;

    /// Check multiple protocols at once
    #[inline]
    pub fn check_protocols_simd(bitmap: u32, protocols: &[u8]) -> Vec<bool> {
        let mut results = Vec::with_capacity(protocols.len());
        for &proto in protocols {
            results.push((bitmap & (1u32 << proto)) != 0);
        }
        results
    }
}