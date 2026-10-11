use criterion::{Criterion, criterion_group, criterion_main};
use oxide_core::{NodeId, OverlayIp, OverlayPrefix};
use oxide_transport::porthopper::{PortHopper, PortHopperConfig};
use oxide_transport::routing::{L1DirectMappedCache, RadixRoutingTable, RcuRouter, RouteTarget};
use std::hint::black_box;
use std::net::Ipv4Addr;

fn bench_routing_lookups(c: &mut Criterion) {
    let mut table = RadixRoutingTable::new();
    let target = RouteTarget::new(NodeId::new()).with_pmtu(1420);

    // Populate routes across diverse subnets
    table.insert(
        OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(100, 64, 0, 0)), 16).unwrap(),
        target,
    );
    table.insert(
        OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(192, 168, 1, 0)), 24).unwrap(),
        target,
    );
    table.insert(
        OverlayPrefix::new(OverlayIp::V4(Ipv4Addr::new(10, 0, 0, 0)), 8).unwrap(),
        target,
    );

    let router = RcuRouter::new();
    router.update(table);

    let test_ip = OverlayIp::V4(Ipv4Addr::new(100, 64, 10, 50));
    let mut l1_cache = L1DirectMappedCache::new();

    // Warm the L1 cache
    router.lookup(test_ip, &mut l1_cache);

    let mut group = c.benchmark_group("routing");

    group.bench_function("l1_cache_hit_lookup", |b| {
        b.iter(|| {
            let res = router.lookup(black_box(test_ip), &mut l1_cache);
            black_box(res);
        })
    });

    group.bench_function("radix_tree_cold_lookup", |b| {
        let t = router.load_table();
        b.iter(|| {
            let res = t.lookup(black_box(test_ip));
            black_box(res);
        })
    });

    group.finish();
}

fn bench_port_hopper(c: &mut Criterion) {
    let hopper = PortHopper::new(PortHopperConfig::default());

    c.bench_function("port_hopper_compute_port", |b| {
        let mut epoch = 1000u64;
        b.iter(|| {
            epoch = epoch.wrapping_add(1);
            let port = hopper.compute_port_for_epoch(black_box(epoch));
            black_box(port);
        })
    });
}

criterion_group!(benches, bench_routing_lookups, bench_port_hopper);
criterion_main!(benches);
