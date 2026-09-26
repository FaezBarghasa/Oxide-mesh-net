//! ACL rule definitions and SIMD-accelerated evaluation

use std::collections::HashMap;
use smallvec::SmallVec;
use oxide_core::{OverlayIp, OverlayPrefix};
use oxide_crypto::keys::KeyFingerprint;
use crate::error::{AclError, Result};
use crate::simd::SimdEvaluator;

/// ACL action
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AclAction {
    Allow,
    Deny,
    Log,
}

/// ACL direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AclDirection {
    Ingress,
    Egress,
    Both,
}

/// Transport protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Protocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Icmpv6 = 58,
    Any = 255,
}

/// Port range
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

impl PortRange {
    pub fn new(start: u16, end: u16) -> Result<Self> {
        if start > end {
            return Err(AclError::InvalidRule("Port range start must be <= end".into()));
        }
        Ok(Self { start, end })
    }

    pub fn single(port: u16) -> Self {
        Self { start: port, end: port }
    }

    pub fn any() -> Self {
        Self { start: 0, end: 65535 }
    }

    pub fn contains(&self, port: u16) -> bool {
        port >= self.start && port <= self.end
    }
}

/// ACL rule with compiled SIMD-friendly representation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AclRule {
    pub id: String,
    pub action: AclAction,
    pub direction: AclDirection,
    pub priority: u32,
    pub log: bool,

    // Source matchers
    pub src_identities: SmallVec<[KeyFingerprint; 4]>,
    pub src_prefixes: SmallVec<[OverlayPrefix; 4]>,

    // Destination matchers
    pub dst_prefixes: SmallVec<[OverlayPrefix; 4]>,

    // Protocol/port matchers
    pub protocols: SmallVec<[Protocol; 4]>,
    pub src_ports: SmallVec<[PortRange; 4]>,
    pub dst_ports: SmallVec<[PortRange; 4]>,

    // Compiled bitmask representation (for SIMD)
    #[serde(skip)]
    pub compiled: Option<CompiledRule>,
}

/// Compiled rule for fast SIMD evaluation
#[derive(Debug, Clone)]
pub struct CompiledRule {
    /// Identity bitmask (if using identity-based matching)
    pub identity_mask: Option<u64>,
    /// Prefix trie for CIDR matching
    pub src_prefix_trie: Option<PrefixTrie>,
    pub dst_prefix_trie: Option<PrefixTrie>,
    /// Protocol bitmap
    pub protocol_bitmap: u32,
    /// Port range bitmaps (64KB = 8192 u64s)
    pub src_port_bitmap: Option<Vec<u64>>,
    pub dst_port_bitmap: Option<Vec<u64>>,
}

/// Prefix trie for CIDR matching
#[derive(Debug, Clone)]
pub struct PrefixTrie {
    root: PrefixNode,
}

#[derive(Debug, Clone)]
struct PrefixNode {
    children: [Option<Box<PrefixNode>>; 2],
    prefixes: SmallVec<[OverlayPrefix; 4]>,
}

impl PrefixTrie {
    fn new() -> Self {
        Self { root: PrefixNode::new() }
    }

    fn insert(&mut self, prefix: OverlayPrefix) {
        let bits = match prefix.addr {
            OverlayIp::V4(addr) => {
                let addr_bits = u32::from(addr);
                let len = prefix.prefix_len as usize;
                (0..len).map(|i| (addr_bits >> (31 - i)) & 1).collect::<Vec<_>>()
            }
            OverlayIp::V6(addr) => {
                let addr_bytes = addr.octets();
                let len = prefix.prefix_len as usize;
                (0..len).map(|i| {
                    let byte_idx = i / 8;
                    let bit_idx = 7 - (i % 8);
                    ((addr_bytes[byte_idx] >> bit_idx) & 1) as u32
                }).collect::<Vec<_>>()
            }
        };

        let mut node = &mut self.root;
        for bit in bits {
            node = node.children[bit as usize].get_or_insert_with(|| Box::new(PrefixNode::new()));
        }
        node.prefixes.push(prefix);
    }

    fn match_ip(&self, ip: OverlayIp) -> SmallVec<[OverlayPrefix; 4]> {
        let bits = match ip {
            OverlayIp::V4(addr) => {
                let addr_bits = u32::from(addr);
                (0..32).map(|i| (addr_bits >> (31 - i)) & 1).collect::<Vec<_>>()
            }
            OverlayIp::V6(addr) => {
                let addr_bytes = addr.octets();
                (0..128).map(|i| {
                    let byte_idx = i / 8;
                    let bit_idx = 7 - (i % 8);
                    ((addr_bytes[byte_idx] >> bit_idx) & 1) as u32
                }).collect::<Vec<_>>()
            }
        };

        let mut results = SmallVec::new();
        let mut node = &self.root;
        results.extend(node.prefixes.iter().cloned());

        for bit in bits {
            if let Some(child) = &node.children[bit as usize] {
                node = child;
                results.extend(node.prefixes.iter().cloned());
            } else {
                break;
            }
        }

        results
    }
}

impl PrefixNode {
    fn new() -> Self {
        Self {
            children: [None, None],
            prefixes: SmallVec::new(),
        }
    }
}

/// Packet metadata for ACL evaluation
#[derive(Debug, Clone, Copy)]
pub struct PacketMeta {
    pub src_ip: OverlayIp,
    pub dst_ip: OverlayIp,
    pub src_identity: Option<KeyFingerprint>,
    pub protocol: Protocol,
    pub src_port: u16,
    pub dst_port: u16,
    pub direction: AclDirection,
}

/// Evaluation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AclResult {
    Allow,
    Deny,
    Log,
}

/// ACL engine with SIMD acceleration
pub struct AclEngine {
    rules: Vec<AclRule>,
    default_action: AclAction,
    simd_evaluator: SimdEvaluator,
    rule_map: HashMap<String, usize>,
}

impl AclEngine {
    pub fn new(rules: Vec<AclRule>, default_action: AclAction) -> Result<Self> {
        let mut engine = Self {
            rules,
            default_action,
            simd_evaluator: SimdEvaluator::new()?,
            rule_map: HashMap::new(),
        };
        engine.compile_rules()?;
        engine.build_rule_map();
        Ok(engine)
    }

    fn compile_rules(&mut self) -> Result<()> {
        // Sort by priority (higher first)
        self.rules.sort_by_key(|r| std::cmp::Reverse(r.priority));

        for rule in &mut self.rules {
            rule.compiled = Some(Self::compile_rule(rule)?);
        }
        Ok(())
    }

    fn compile_rule(rule: &AclRule) -> Result<CompiledRule> {
        let mut compiled = CompiledRule {
            identity_mask: None,
            src_prefix_trie: None,
            dst_prefix_trie: None,
            protocol_bitmap: 0,
            src_port_bitmap: None,
            dst_port_bitmap: None,
        };

        // Compile protocol bitmap
        for proto in &rule.protocols {
            compiled.protocol_bitmap |= 1 << (*proto as u8);
        }
        if rule.protocols.is_empty() {
            compiled.protocol_bitmap = u32::MAX;
        }

        // Compile prefix tries
        if !rule.src_prefixes.is_empty() {
            let mut trie = PrefixTrie::new();
            for prefix in &rule.src_prefixes {
                trie.insert(*prefix);
            }
            compiled.src_prefix_trie = Some(trie);
        }

        if !rule.dst_prefixes.is_empty() {
            let mut trie = PrefixTrie::new();
            for prefix in &rule.dst_prefixes {
                trie.insert(*prefix);
            }
            compiled.dst_prefix_trie = Some(trie);
        }

        // Compile port bitmaps (for SIMD)
        if !rule.src_ports.is_empty() {
            let mut bitmap = vec![0u64; 1024]; // 65536 bits = 1024 u64s
            for range in &rule.src_ports {
                for port in range.start..=range.end {
                    let idx = port as usize / 64;
                    let bit = port as usize % 64;
                    bitmap[idx] |= 1u64 << bit;
                }
            }
            compiled.src_port_bitmap = Some(bitmap);
        }

        if !rule.dst_ports.is_empty() {
            let mut bitmap = vec![0u64; 1024];
            for range in &rule.dst_ports {
                for port in range.start..=range.end {
                    let idx = port as usize / 64;
                    let bit = port as usize % 64;
                    bitmap[idx] |= 1u64 << bit;
                }
            }
            compiled.dst_port_bitmap = Some(bitmap);
        }

        // Identity mask (if using exact identity matching)
        if !rule.src_identities.is_empty() {
            // Would map identities to bit positions
            // For now, use a simple hash-based approach
            compiled.identity_mask = Some(0); // Placeholder
        }

        Ok(compiled)
    }

    fn build_rule_map(&mut self) {
        for (idx, rule) in self.rules.iter().enumerate() {
            self.rule_map.insert(rule.id.clone(), idx);
        }
    }

    /// Evaluate a packet against ACL rules
    pub fn evaluate(&self, meta: &PacketMeta) -> AclResult {
        // Try SIMD fast path first
        if let Some(result) = self.simd_evaluator.evaluate_batch(&self.rules, meta) {
            return result;
        }

        // Fallback to scalar evaluation
        self.evaluate_scalar(meta)
    }

    /// Scalar evaluation fallback
    fn evaluate_scalar(&self, meta: &PacketMeta) -> AclResult {
        for rule in &self.rules {
            if self.match_rule(rule, meta) {
                return match rule.action {
                    AclAction::Allow => AclResult::Allow,
                    AclAction::Deny => AclResult::Deny,
                    AclAction::Log => AclResult::Log,
                };
            }
        }
        match self.default_action {
            AclAction::Allow => AclResult::Allow,
            AclAction::Deny => AclResult::Deny,
            AclAction::Log => AclResult::Log,
        }
    }

    fn match_rule(&self, rule: &AclRule, meta: &PacketMeta) -> bool {
        // Check direction
        if rule.direction != AclDirection::Both && rule.direction != meta.direction {
            return false;
        }

        // Check source identity
        if !rule.src_identities.is_empty() {
            if let Some(identity) = meta.src_identity {
                if !rule.src_identities.iter().any(|id| *id == identity) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check source prefix
        if !rule.src_prefixes.is_empty() {
            if let Some(compiled) = &rule.compiled {
                if let Some(trie) = &compiled.src_prefix_trie {
                    let matches = trie.match_ip(meta.src_ip);
                    if matches.is_empty() {
                        return false;
                    }
                }
            }
        }

        // Check destination prefix
        if !rule.dst_prefixes.is_empty() {
            if let Some(compiled) = &rule.compiled {
                if let Some(trie) = &compiled.dst_prefix_trie {
                    let matches = trie.match_ip(meta.dst_ip);
                    if matches.is_empty() {
                        return false;
                    }
                }
            }
        }

        // Check protocol
        if !rule.protocols.is_empty() && !rule.protocols.iter().any(|p| *p == meta.protocol) {
            return false;
        }

        // Check source port
        if !rule.src_ports.is_empty() {
            if !rule.src_ports.iter().any(|range| range.contains(meta.src_port)) {
                return false;
            }
        }

        // Check destination port
        if !rule.dst_ports.is_empty() {
            if !rule.dst_ports.iter().any(|range| range.contains(meta.dst_port)) {
                return false;
            }
        }

        true
    }

    /// Add a rule dynamically
    pub fn add_rule(&mut self, rule: AclRule) -> Result<()> {
        let compiled = Self::compile_rule(&rule)?;
        let mut rule = rule;
        rule.compiled = Some(compiled);
        
        // Insert maintaining priority order
        let insert_idx = self.rules.iter().position(|r| r.priority < rule.priority)
            .unwrap_or(self.rules.len());
        self.rules.insert(insert_idx, rule);
        self.rebuild_rule_map();
        Ok(())
    }

    /// Remove a rule by ID
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<()> {
        if let Some(idx) = self.rule_map.get(rule_id).copied() {
            self.rules.remove(idx);
            self.rebuild_rule_map();
            Ok(())
        } else {
            Err(AclError::RuleNotFound(rule_id.into()))
        }
    }

    fn rebuild_rule_map(&mut self) {
        self.rule_map.clear();
        for (idx, rule) in self.rules.iter().enumerate() {
            self.rule_map.insert(rule.id.clone(), idx);
        }
    }

    /// Get all rules
    pub fn rules(&self) -> &[AclRule] {
        &self.rules
    }
}

impl Default for AclEngine {
    fn default() -> Self {
        Self::new(vec![], AclAction::Allow).unwrap()
    }
}