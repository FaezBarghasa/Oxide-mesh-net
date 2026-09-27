//! RFC 8899 Datagram Packetization Layer Path MTU Discovery (DPLPMTUD)
//!
//! Provides automated and continuous path capacity measurement between overlay mesh peers
//! over QUIC datagrams, dynamically adjusting session PMTU and avoiding blackholing.

use std::time::{Duration, Instant};

/// Standard probing steps for DPLPMTUD search
pub const DEFAULT_PROBE_STEPS: &[u16] = &[1280, 1360, 1420, 1472, 1500, 9000];

/// DPLPMTUD State Machine State (RFC 8899 Section 5.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DplpmtudPhase {
    /// Validating minimum base PMTU (1280 bytes)
    BaseSearch,
    /// Actively probing upward step sizes
    Searching,
    /// Search complete; periodically verifying PMTU validity
    SearchComplete,
    /// Path error occurred; down-stepping PMTU
    Error,
}

/// DPLPMTUD Configuration
#[derive(Debug, Clone)]
pub struct DplpmtudConfig {
    pub min_pmtu: u16,
    pub max_pmtu: u16,
    pub probe_timeout: Duration,
    pub verification_interval: Duration,
    pub max_probe_attempts: u8,
}

impl Default for DplpmtudConfig {
    fn default() -> Self {
        Self {
            min_pmtu: 1280,
            max_pmtu: 1500,
            probe_timeout: Duration::from_millis(1000),
            verification_interval: Duration::from_secs(60),
            max_probe_attempts: 3,
        }
    }
}

/// Dynamic DPLPMTUD Probing Engine per Peer Session
#[derive(Debug, Clone)]
pub struct DplpmtudEngine {
    config: DplpmtudConfig,
    phase: DplpmtudPhase,
    confirmed_pmtu: u16,
    probed_size: u16,
    step_index: usize,
    probe_attempts: u8,
    last_probe_sent: Option<Instant>,
    last_confirmed: Instant,
}

impl DplpmtudEngine {
    pub fn new(config: DplpmtudConfig) -> Self {
        let min_pmtu = config.min_pmtu;
        Self {
            config,
            phase: DplpmtudPhase::BaseSearch,
            confirmed_pmtu: min_pmtu,
            probed_size: min_pmtu,
            step_index: 0,
            probe_attempts: 0,
            last_probe_sent: None,
            last_confirmed: Instant::now(),
        }
    }

    /// Return the currently confirmed active Path MTU
    #[inline]
    pub fn confirmed_pmtu(&self) -> u16 {
        self.confirmed_pmtu
    }

    /// Return current phase
    #[inline]
    pub fn phase(&self) -> DplpmtudPhase {
        self.phase
    }

    /// Check if a new probe frame should be dispatched
    pub fn poll_probe(&mut self, now: Instant) -> Option<u16> {
        match self.phase {
            DplpmtudPhase::BaseSearch => {
                if let Some(sent) = self.last_probe_sent
                    && now.duration_since(sent) < self.config.probe_timeout
                {
                    return None;
                }
                self.probed_size = self.config.min_pmtu;
                self.last_probe_sent = Some(now);
                Some(self.probed_size)
            }
            DplpmtudPhase::Searching => {
                if let Some(sent) = self.last_probe_sent
                    && now.duration_since(sent) < self.config.probe_timeout
                {
                    return None;
                }
                if self.step_index < DEFAULT_PROBE_STEPS.len() {
                    let target = DEFAULT_PROBE_STEPS[self.step_index].min(self.config.max_pmtu);
                    if target <= self.confirmed_pmtu {
                        self.step_index += 1;
                        return self.poll_probe(now);
                    }
                    self.probed_size = target;
                    self.last_probe_sent = Some(now);
                    Some(self.probed_size)
                } else {
                    self.phase = DplpmtudPhase::SearchComplete;
                    None
                }
            }
            DplpmtudPhase::SearchComplete => {
                if now.duration_since(self.last_confirmed) >= self.config.verification_interval {
                    // Periodic verification of the confirmed PMTU
                    self.last_probe_sent = Some(now);
                    self.last_confirmed = now;
                    Some(self.confirmed_pmtu)
                } else {
                    None
                }
            }
            DplpmtudPhase::Error => {
                self.confirmed_pmtu = self.config.min_pmtu;
                self.phase = DplpmtudPhase::BaseSearch;
                self.step_index = 0;
                self.probe_attempts = 0;
                self.poll_probe(now)
            }
        }
    }

    /// Handle probe acknowledgment
    pub fn on_probe_acked(&mut self, acked_size: u16, now: Instant) {
        if acked_size >= self.confirmed_pmtu {
            self.confirmed_pmtu = acked_size;
            self.last_confirmed = now;
            self.probe_attempts = 0;
            self.last_probe_sent = None;

            match self.phase {
                DplpmtudPhase::BaseSearch => {
                    self.phase = DplpmtudPhase::Searching;
                    self.step_index = 0;
                }
                DplpmtudPhase::Searching => {
                    self.step_index += 1;
                    if self.step_index >= DEFAULT_PROBE_STEPS.len()
                        || DEFAULT_PROBE_STEPS[self.step_index] > self.config.max_pmtu
                    {
                        self.phase = DplpmtudPhase::SearchComplete;
                    }
                }
                DplpmtudPhase::SearchComplete | DplpmtudPhase::Error => {}
            }
        }
    }

    /// Handle probe timeout / loss
    pub fn on_probe_lost(&mut self, lost_size: u16) {
        if lost_size == self.probed_size {
            self.probe_attempts += 1;
            if self.probe_attempts >= self.config.max_probe_attempts {
                match self.phase {
                    DplpmtudPhase::BaseSearch => {
                        self.phase = DplpmtudPhase::Error;
                    }
                    DplpmtudPhase::Searching => {
                        // Stop searching; confirmed_pmtu is the maximum valid PMTU
                        self.phase = DplpmtudPhase::SearchComplete;
                    }
                    DplpmtudPhase::SearchComplete => {
                        // Verification failed, revert to search
                        self.phase = DplpmtudPhase::Searching;
                        self.step_index = 0;
                    }
                    DplpmtudPhase::Error => {}
                }
                self.probe_attempts = 0;
                self.last_probe_sent = None;
            }
        }
    }

    /// Explicit ICMP Packet-Too-Big notification handler
    pub fn on_packet_too_big(&mut self, advertised_mtu: u16) {
        let clamped = advertised_mtu.clamp(self.config.min_pmtu, self.config.max_pmtu);
        if clamped < self.confirmed_pmtu {
            self.confirmed_pmtu = clamped;
            self.phase = DplpmtudPhase::SearchComplete;
            self.last_probe_sent = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dplpmtud_lifecycle() {
        let config = DplpmtudConfig {
            min_pmtu: 1280,
            max_pmtu: 1500,
            max_probe_attempts: 2,
            ..Default::default()
        };
        let mut engine = DplpmtudEngine::new(config);
        let mut now = Instant::now();

        // 1. Initial base probe (1280)
        let p1 = engine.poll_probe(now).unwrap();
        assert_eq!(p1, 1280);
        assert_eq!(engine.phase(), DplpmtudPhase::BaseSearch);

        // 2. Ack 1280 -> Enters Searching
        engine.on_probe_acked(1280, now);
        assert_eq!(engine.phase(), DplpmtudPhase::Searching);

        // 3. Probes 1360
        let p2 = engine.poll_probe(now).unwrap();
        assert_eq!(p2, 1360);
        engine.on_probe_acked(1360, now);

        // 4. Probes 1420
        let p3 = engine.poll_probe(now).unwrap();
        assert_eq!(p3, 1420);
        engine.on_probe_acked(1420, now);

        // 5. Probes 1472, but simulated loss
        let p4 = engine.poll_probe(now).unwrap();
        assert_eq!(p4, 1472);
        engine.on_probe_lost(1472); // attempt 1

        now += Duration::from_millis(1500);
        let _ = engine.poll_probe(now);
        engine.on_probe_lost(1472); // attempt 2 -> stops search

        assert_eq!(engine.phase(), DplpmtudPhase::SearchComplete);
        assert_eq!(engine.confirmed_pmtu(), 1420);
    }
}
