// projects/libraries/symbolic/benches/symbolic_bench.rs
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use symbolic::rules::RulesEngine;
use symbolic::validator::CodeValidator;

fn bench_rules_generate_struct(c: &mut Criterion) {
    let engine = RulesEngine::new().expect("Failed to create RulesEngine");
    let prompt = "create struct Order with id and total";

    c.bench_function("rules_generate_struct", |b| {
        b.iter(|| {
            let result = engine.generate(black_box(prompt), None);
            black_box(result).ok();
        })
    });
}

fn bench_rules_match_confidence(c: &mut Criterion) {
    let engine = RulesEngine::new().expect("Failed to create RulesEngine");
    let prompt = "create function calculate";

    c.bench_function("rules_match_confidence", |b| {
        b.iter(|| {
            let confidence = engine.match_confidence(black_box(prompt));
            black_box(confidence);
        })
    });
}

fn bench_validate_code_valid(c: &mut Criterion) {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "fn main() { let x = 1 + 2; println!(\"{}\", x); }";

    c.bench_function("validate_code_valid", |b| {
        b.iter(|| {
            let result = validator.validate(black_box(code));
            black_box(result).ok();
        })
    });
}

fn bench_validate_code_invalid(c: &mut Criterion) {
    let validator = CodeValidator::new().expect("Failed to create CodeValidator");
    let code = "fn main( { let x = 1 + ; }";

    c.bench_function("validate_code_invalid", |b| {
        b.iter(|| {
            let result = validator.validate(black_box(code));
            black_box(result).ok();
        })
    });
}

criterion_group!(
    benches,
    bench_rules_generate_struct,
    bench_rules_match_confidence,
    bench_validate_code_valid,
    bench_validate_code_invalid
);
criterion_main!(benches);
