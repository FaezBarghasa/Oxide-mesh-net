//! Transport Circuit Breaker & Actix Web WSS Fallback Mechanism
//!
//! Continuously monitors UDP packet loss over a sliding window. If UDP is completely blackholed
//! (>85% loss for >5s), trips the circuit breaker to fall back to an Actix Web TLS 1.3 WebSocket
//! reverse-tunnel (/ws/v1/telemetry) without dropping active overlay TCP connections.

use std::time::{Duration, Instant};

/// Current status of the transport circuit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitStatus {
    /// Normal primary QUIC/UDP datagram transport
    PrimaryUdp,
    /// Degraded state; verifying UDP connectivity
    Degraded,
    /// Fallback active; routing packets over Actix Web WSS tunnel
    FallbackWss,
}

/// Circuit Breaker Configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub loss_threshold_percent: f32, // e.g. 85.0%
    pub trigger_duration: Duration,  // e.g. 5 seconds
    pub recovery_success_threshold: usize, // e.g. 5 successful pings
    pub probe_interval: Duration,    // e.g. 2 seconds
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            loss_threshold_percent: 85.0,
            trigger_duration: Duration::from_secs(5),
            recovery_success_threshold: 5,
            probe_interval: Duration::from_secs(2),
        }
    }
}

/// Dynamic Transport Circuit Breaker
pub struct TransportCircuitBreaker {
    config: CircuitBreakerConfig,
    status: CircuitStatus,
    sent_packets: u64,
    lost_packets: u64,
    high_loss_since: Option<Instant>,
    consecutive_recovery_pings: usize,
    last_probe_sent: Instant,
}

impl TransportCircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            status: CircuitStatus::PrimaryUdp,
            sent_packets: 0,
            lost_packets: 0,
            high_loss_since: None,
            consecutive_recovery_pings: 0,
            last_probe_sent: Instant::now(),
        }
    }

    pub fn status(&self) -> CircuitStatus {
        self.status
    }

    /// Record a packet transmission outcome
    pub fn record_sample(&mut self, delivered: bool, now: Instant) {
        self.sent_packets += 1;
        if !delivered {
            self.lost_packets += 1;
        }

        // Evaluate every 50 packets or upon loss
        if self.sent_packets >= 50 {
            let loss_rate = (self.lost_packets as f32 / self.sent_packets as f32) * 100.0;
            if loss_rate >= self.config.loss_threshold_percent {
                if self.high_loss_since.is_none() {
                    self.high_loss_since = Some(now);
                } else if let Some(since) = self.high_loss_since {
                    if now.duration_since(since) >= self.config.trigger_duration {
                        self.status = CircuitStatus::FallbackWss;
                    }
                }
            } else {
                self.high_loss_since = None;
            }

            // Reset sample window
            self.sent_packets = 0;
            self.lost_packets = 0;
        }
    }

    /// Record background UDP recovery ping result while in fallback
    pub fn record_recovery_ping(&mut self, success: bool) {
        if self.status == CircuitStatus::FallbackWss {
            if success {
                self.consecutive_recovery_pings += 1;
                if self.consecutive_recovery_pings >= self.config.recovery_success_threshold {
                    // Restore UDP primary transport
                    self.status = CircuitStatus::PrimaryUdp;
                    self.consecutive_recovery_pings = 0;
                    self.high_loss_since = None;
                }
            } else {
                self.consecutive_recovery_pings = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_trips_to_fallback_and_recovers() {
        let config = CircuitBreakerConfig {
            loss_threshold_percent: 80.0,
            trigger_duration: Duration::from_millis(50),
            recovery_success_threshold: 3,
            ..Default::default()
        };
        let mut breaker = TransportCircuitBreaker::new(config);
        let mut now = Instant::now();

        // Feed 50 packets with 90% loss
        for i in 0..50 {
            breaker.record_sample(i >= 45, now);
        }

        // Advance time past trigger duration
        now += Duration::from_millis(100);

        // Feed another 50 packets with 90% loss
        for i in 0..50 {
            breaker.record_sample(i >= 45, now);
        }

        assert_eq!(breaker.status(), CircuitStatus::FallbackWss);

        // Feed 3 successful recovery pings
        breaker.record_recovery_ping(true);
        breaker.record_recovery_ping(true);
        breaker.record_recovery_ping(true);

        assert_eq!(breaker.status(), CircuitStatus::PrimaryUdp);
    }
}
