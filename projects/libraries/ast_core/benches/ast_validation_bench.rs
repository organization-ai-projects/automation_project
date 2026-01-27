use ast_core::{AstKind, AstNode, ValidateLimits};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_ast_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ast_validation");

    for size in [10, 100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut root = AstNode::new(AstKind::Array(vec![]));
                if let AstKind::Array(ref mut vec) = root.kind {
                    for i in 0..size {
                        let child = AstNode::new(AstKind::Number(i.into()));
                        vec.push(child);
                    }
                }
                let _ = black_box(root.validate_with(&ValidateLimits::default()));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_ast_validation);
criterion_main!(benches);
