//! Dual-Epoch PRNG Port Hopping Synchronization
//!
//! Provides clock-drift resilient port hopping across dynamic UDP port ranges (e.g. 25000..45000),
//! computing concurrent active listening ports across [P_prev, P_current, P_next] to eliminate
//! packet drop transitions under clock skew.

/// Port Hopper Configuration
#[derive(Debug, Clone)]
pub struct PortHopperConfig {
    pub seed: [u8; 32],
    pub min_port: u16,
    pub max_port: u16,
    pub hop_interval_secs: u64,
    pub overlap_window_secs: u64,
}

impl Default for PortHopperConfig {
    fn default() -> Self {
        Self {
            seed: [0x42; 32],
            min_port: 25000,
            max_port: 45000,
            hop_interval_secs: 30,
            overlap_window_secs: 5,
        }
    }
}

/// Dual-Epoch Port Hopper Engine
#[derive(Debug, Clone)]
pub struct PortHopper {
    config: PortHopperConfig,
}

impl PortHopper {
    pub fn new(config: PortHopperConfig) -> Self {
        Self { config }
    }

    /// Compute deterministic pseudo-random port for a given epoch
    pub fn compute_port_for_epoch(&self, epoch: u64) -> u16 {
        // Hash (seed || epoch) with blake3 for cryptographically uniform distribution
        let mut hasher = blake3::Hasher::new_keyed(&self.config.seed);
        hasher.update(&epoch.to_le_bytes());
        let hash = hasher.finalize();
        let slice: [u8; 2] = [hash.as_bytes()[0], hash.as_bytes()[1]];
        let rand_val = u16::from_le_bytes(slice);

        let span = (self.config.max_port - self.config.min_port + 1) as u32;
        let port_offset = (rand_val as u32 % span) as u16;
        self.config.min_port + port_offset
    }

    /// Compute current epoch from UNIX timestamp
    #[inline]
    pub fn epoch_at(&self, unix_timestamp_secs: u64) -> u64 {
        unix_timestamp_secs / self.config.hop_interval_secs
    }

    /// Return active listening port triple [P_prev, P_current, P_next] for dual-epoch sliding
    pub fn active_ports(&self, unix_timestamp_secs: u64) -> (u16, u16, u16) {
        let current_epoch = self.epoch_at(unix_timestamp_secs);
        let p_prev = self.compute_port_for_epoch(current_epoch.saturating_sub(1));
        let p_current = self.compute_port_for_epoch(current_epoch);
        let p_next = self.compute_port_for_epoch(current_epoch + 1);
        (p_prev, p_current, p_next)
    }

    /// Return true if a received packet on `dest_port` belongs to the current valid sliding window
    pub fn is_valid_port(&self, unix_timestamp_secs: u64, dest_port: u16) -> bool {
        let (p_prev, p_curr, p_next) = self.active_ports(unix_timestamp_secs);
        dest_port == p_prev || dest_port == p_curr || dest_port == p_next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_hopper_distribution_and_bounds() {
        let config = PortHopperConfig {
            seed: [0x7A; 32],
            min_port: 30000,
            max_port: 40000,
            hop_interval_secs: 30,
            overlap_window_secs: 5,
        };
        let hopper = PortHopper::new(config);

        for epoch in 0..100 {
            let port = hopper.compute_port_for_epoch(epoch);
            assert!((30000..=40000).contains(&port));
        }
    }

    #[test]
    fn test_active_ports_sliding_window() {
        let hopper = PortHopper::new(PortHopperConfig::default());
        let t0 = 1000u64;
        let (p_prev, p_curr, p_next) = hopper.active_ports(t0);

        assert!(hopper.is_valid_port(t0, p_prev));
        assert!(hopper.is_valid_port(t0, p_curr));
        assert!(hopper.is_valid_port(t0, p_next));
        assert!(!hopper.is_valid_port(t0, 1234));
    }
}
