# routing

## Classs

- [L1DirectMappedCache](L1DirectMappedCache.md) — Thread-local L1 Direct-Mapped Routing Cache
- [RadixRoutingTable](RadixRoutingTable.md) — Immutable Prefix Radix Routing Table
- [RcuRouter](RcuRouter.md) — Global RCU Router Engine
- [RouteEntry](RouteEntry.md) — An entry in the immutable radix routing table
- [RouteTarget](RouteTarget.md) — Target destination for routed traffic

## Functions

- [all_routes](all_routes.md) — Return all active routes in the table
- [all_routes](all_routes_1.md) — Return all active routes in the table
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [from_table](from_table.md)
- [from_table](from_table_1.md)
- [get](get.md) — [inline]
- [get](get_1.md) — [inline]
- [hash_index](hash_index.md) — [inline]
- [hash_index](hash_index_1.md) — [inline]
- [insert](insert.md) — Insert or replace a route entry and sort by longest prefix length descending
- [insert](insert_1.md) — Insert or replace a route entry and sort by longest prefix length descending
- [insert](insert_2.md) — [inline]
- [insert](insert_3.md) — [inline]
- [invalidate](invalidate.md)
- [invalidate](invalidate_1.md)
- [load_table](load_table.md) — Lock-free load of the entire routing table Arc
- [load_table](load_table_1.md) — Lock-free load of the entire routing table Arc
- [lookup](lookup.md) — Longest Prefix Match lookup
- [lookup](lookup_1.md) — Longest Prefix Match lookup
- [lookup](lookup_2.md) — Fast-path lookup combining L1 thread-local cache and lock-free RCU load
- [lookup](lookup_3.md) — Fast-path lookup combining L1 thread-local cache and lock-free RCU load
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new](new_6.md)
- [new](new_7.md)
- [remove](remove.md) — Remove a prefix route
- [remove](remove_1.md) — Remove a prefix route
- [test_radix_routing_longest_prefix_match](test_radix_routing_longest_prefix_match.md) — [test]
- [test_rcu_router_with_l1_cache](test_rcu_router_with_l1_cache.md) — [test]
- [update](update.md) — Atomic RCU swap of a newly compiled routing table
- [update](update_1.md) — Atomic RCU swap of a newly compiled routing table
- [with_pmtu](with_pmtu.md)
- [with_pmtu](with_pmtu_1.md)
