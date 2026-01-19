// projects/libraries/ast_core/benches/ast_bench.rs
use ast_core::{AstKind, AstNode};
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

fn bench_ast_node_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ast_node_creation");

    for size in [1, 10, 100, 1_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                for _ in 0..size {
                    let node = AstNode::new(AstKind::Null);
                    black_box(node);
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_ast_node_creation);
criterion_main!(benches);
