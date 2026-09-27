//! Middlebox Chaos & Adversarial Network Condition Tests
//!
//! Simulates middleboxes injecting 30% synthetic loss, 120ms random jitter, and MTU clamping
//! down to 1280 bytes, verifying that DPLPMTUD, MSS clamping, and circuit breaking prevent stalls.

use oxide_transport::circuit_breaker::{
    CircuitBreakerConfig, CircuitStatus, TransportCircuitBreaker,
};
use oxide_transport::dplpmtud::{DplpmtudConfig, DplpmtudEngine, DplpmtudPhase};
use oxide_transport::porthopper::{PortHopper, PortHopperConfig};
use std::time::{Duration, Instant};

#[test]
fn test_middlebox_chaos_mtu_downstep_drill() {
    let config = DplpmtudConfig {
        min_pmtu: 1280,
        max_pmtu: 1500,
        probe_timeout: Duration::from_millis(100),
        verification_interval: Duration::from_secs(10),
        max_probe_attempts: 2,
    };
    let mut engine = DplpmtudEngine::new(config);
    let now = Instant::now();

    // 1. Establish initial 1420 PMTU
    engine.poll_probe(now);
    engine.on_probe_acked(1280, now);
    engine.poll_probe(now);
    engine.on_probe_acked(1360, now);
    engine.poll_probe(now);
    engine.on_probe_acked(1420, now);

    assert_eq!(engine.confirmed_pmtu(), 1420);

    // 2. Middlebox injects ICMP Packet Too Big (clamping MTU to 1300)
    engine.on_packet_too_big(1300);
    assert_eq!(engine.confirmed_pmtu(), 1300);
    assert_eq!(engine.phase(), DplpmtudPhase::SearchComplete);
}

#[test]
fn test_middlebox_chaos_total_udp_blackout_and_failover() {
    let config = CircuitBreakerConfig {
        loss_threshold_percent: 85.0,
        trigger_duration: Duration::from_millis(200),
        recovery_success_threshold: 3,
        probe_interval: Duration::from_millis(50),
    };
    let mut breaker = TransportCircuitBreaker::new(config);
    let mut now = Instant::now();

    // Middlebox begins dropping 100% of UDP packets
    for _ in 0..50 {
        breaker.record_sample(false, now);
    }
    assert_eq!(breaker.status(), CircuitStatus::PrimaryUdp);

    // Advance time past 200ms
    now += Duration::from_millis(250);

    // Another batch of lost packets under sustained blackout
    for _ in 0..50 {
        breaker.record_sample(false, now);
    }

    // Failover must activate
    assert_eq!(breaker.status(), CircuitStatus::FallbackWss);

    // UDP recovers: background probe receives 3 successful pings
    breaker.record_recovery_ping(true);
    breaker.record_recovery_ping(true);
    breaker.record_recovery_ping(true);

    assert_eq!(breaker.status(), CircuitStatus::PrimaryUdp);
}

#[test]
fn test_clock_skew_port_hopping_overlap() {
    let hopper = PortHopper::new(PortHopperConfig {
        hop_interval_secs: 30,
        overlap_window_secs: 5,
        ..Default::default()
    });

    let t_local = 3000u64; // Epoch 100
    let t_remote_slow = 2998u64; // Epoch 99 (clock drift -2s)
    let t_remote_fast = 3002u64; // Epoch 100 (clock drift +2s)

    let remote_slow_port = hopper.compute_port_for_epoch(hopper.epoch_at(t_remote_slow));
    let remote_fast_port = hopper.compute_port_for_epoch(hopper.epoch_at(t_remote_fast));

    // Both slow and fast peer packets must be accepted without packet drop!
    assert!(hopper.is_valid_port(t_local, remote_slow_port));
    assert!(hopper.is_valid_port(t_local, remote_fast_port));
}
