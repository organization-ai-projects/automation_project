//! Benchmarks for hybrid_arena.
//!
//! Run with: cargo bench -p hybrid_arena
// projects/libraries/hybrid_arena/benches/arena_bench.rs
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use hybrid_arena::{BumpArena, Id, SlotArena};

// ============================================================================
// Allocation benchmarks
// ============================================================================

fn bench_bump_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_alloc");

    for size in [100, 1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
                for i in 0..size {
                    black_box(arena.alloc(i as u64).expect("Allocation failed"));
                }
                arena
            });
        });
    }
    group.finish();
}

fn bench_slot_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_alloc");

    for size in [100, 1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
                for i in 0..size {
                    black_box(arena.alloc(i as u64).expect("Allocation failed"));
                }
                arena
            });
        });
    }
    group.finish();
}

fn bench_bump_alloc_extend(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_alloc_extend");

    for size in [100, 1_000, 10_000, 100_000] {
        let data: Vec<u64> = (0..size).map(|i| i as u64).collect();
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| {
                let mut arena: BumpArena<u64> = BumpArena::with_capacity(data.len());
                black_box(
                    arena
                        .alloc_extend(data.iter().copied())
                        .expect("Allocation failed"),
                );
                arena
            });
        });
    }
    group.finish();
}

// ============================================================================
// Access benchmarks
// ============================================================================

fn bench_bump_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_get");

    for size in [100, 1_000, 10_000, 100_000] {
        let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena.get(*id).expect("Failed to retrieve value from arena");
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_slot_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_get");

    for size in [100, 1_000, 10_000, 100_000] {
        let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena.get(*id).expect("id present");
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_bump_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_index");

    for size in [100, 1_000, 10_000, 100_000] {
        let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena[*id];
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Access comparison benchmarks (get vs index)
// ============================================================================

fn bench_bump_access_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_access_compare");

    for size in [1_000, 10_000, 100_000] {
        let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));

        // Safe get()
        group.bench_with_input(BenchmarkId::new("get", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena.get(*id).expect("Failed to retrieve value from arena");
                }
                black_box(sum)
            });
        });

        // Index operator
        group.bench_with_input(BenchmarkId::new("index", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena[*id];
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_slot_access_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_access_compare");

    for size in [1_000, 10_000, 100_000] {
        let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));

        // Safe get()
        group.bench_with_input(BenchmarkId::new("get", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena.get(*id).expect("id present");
                }
                black_box(sum)
            });
        });

        // Index operator
        group.bench_with_input(BenchmarkId::new("index", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena[*id];
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Iteration benchmarks
// ============================================================================

fn bench_bump_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_iter");

    for size in [100, 1_000, 10_000, 100_000] {
        let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
        for i in 0..size {
            arena.alloc(i as u64).expect("alloc");
        }

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &arena, |b, arena| {
            b.iter(|| {
                let sum: u64 = arena.iter().sum();
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_slot_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_iter");

    for size in [100, 1_000, 10_000, 100_000] {
        let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
        for i in 0..size {
            arena.alloc(i as u64).expect("alloc");
        }

        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &arena, |b, arena| {
            b.iter(|| {
                let sum: u64 = arena.iter().copied().sum();
                black_box(sum)
            });
        });
    }
    group.finish();
}

// ============================================================================
// SlotArena-specific benchmarks (remove/reuse)
// ============================================================================

fn bench_slot_remove_realloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_remove_realloc");

    for size in [100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64 * 2)); // remove + realloc
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
                    let ids: Vec<Id<u64>> = (0..size)
                        .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
                        .collect();
                    (arena, ids)
                },
                |(mut arena, ids)| {
                    // Remove all
                    for id in &ids {
                        arena.remove(*id);
                    }
                    // Reallocate all (reuses slots)
                    for i in 0..ids.len() {
                        black_box(arena.alloc(i as u64).expect("Allocation failed"));
                    }
                    arena
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_slot_retain(c: &mut Criterion) {
    let mut group = c.benchmark_group("slot_retain");

    for size in [100, 1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let mut arena: SlotArena<u64> = SlotArena::with_capacity(size);
                    for i in 0..size {
                        arena.alloc(i as u64).expect("alloc");
                    }
                    arena
                },
                |mut arena| {
                    arena.retain(|_, v| *v % 2 == 0);
                    arena
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

// ============================================================================
// ID benchmarks
// ============================================================================

fn bench_id_pack_unpack(c: &mut Criterion) {
    c.bench_function("id_pack_unpack", |b| {
        b.iter(|| {
            for i in 0..10_000u32 {
                let id: Id<()> = Id::new(i, i.wrapping_mul(7));
                black_box(id.index());
                black_box(id.generation());
            }
        });
    });
}

fn bench_bump_safe_vs_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("bump_safe_vs_unsafe");

    for size in [1_000, 10_000, 100_000] {
        let mut arena: BumpArena<u64> = BumpArena::with_capacity(size);
        let ids: Vec<Id<u64>> = (0..size)
            .map(|i| arena.alloc(i as u64).expect("Allocation failed"))
            .collect();

        group.throughput(Throughput::Elements(size as u64));

        // Safe get()
        group.bench_with_input(BenchmarkId::new("get_safe", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    sum += arena.get_safe(*id).expect("id present");
                }
                black_box(sum);
            });
        });

        // Unsafe get_unchecked_benchmark()
        group.bench_with_input(BenchmarkId::new("get_unchecked", size), &ids, |b, ids| {
            b.iter(|| {
                let mut sum = 0u64;
                for id in ids {
                    unsafe {
                        sum += *arena.items.get_unchecked(id.index() as usize);
                    }
                }
                black_box(sum);
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_bump_alloc,
    bench_slot_alloc,
    bench_bump_alloc_extend,
    bench_bump_get,
    bench_slot_get,
    bench_bump_index,
    bench_bump_access_compare,
    bench_slot_access_compare,
    bench_bump_iter,
    bench_slot_iter,
    bench_slot_remove_realloc,
    bench_slot_retain,
    bench_id_pack_unpack,
    bench_bump_safe_vs_unsafe,
);

criterion_main!(benches);
