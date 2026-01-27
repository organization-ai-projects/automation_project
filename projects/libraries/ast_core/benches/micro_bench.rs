use criterion::{Criterion, criterion_group, criterion_main};
use hybrid_arena::SlotArena;
use std::hint::black_box;

use ast_core::{AstBuilder, AstKey, AstNode, past};

fn bench_past_object_1000(c: &mut Criterion) {
    let mut arena = SlotArena::with_capacity(1000);

    c.bench_function("past_object_1000", |b| {
        b.iter(|| {
            arena.drain().for_each(drop);

            for i in 0..1000u32 {
                let node = past!({
                    id: (i),
                    name: ("x"),
                    ok: true,
                    value: (i as i64),
                    nested: { a: 1, b: 2 }
                });
                arena.alloc(node).expect("Allocation in the arena failed");
            }

            black_box(arena.len());
        })
    });
}

fn bench_builder_object_1000(c: &mut Criterion) {
    let mut arena = SlotArena::with_capacity(1000);

    c.bench_function("builder_object_1000", |b| {
        b.iter(|| {
            arena.drain().for_each(drop);

            for i in 0..1000u32 {
                let fields: Vec<(AstKey, AstNode)> = vec![
                    (AstKey::from("id"), AstBuilder::from(i)),
                    (AstKey::from("name"), AstBuilder::from("x")),
                    (AstKey::from("ok"), AstBuilder::bool(true)),
                    (AstKey::from("value"), AstBuilder::from(i as i64)),
                    (
                        AstKey::from("nested"),
                        AstBuilder::object(vec![
                            (AstKey::from("a"), AstBuilder::from(1)),
                            (AstKey::from("b"), AstBuilder::from(2)),
                        ]),
                    ),
                ];
                let node = AstBuilder::object(fields);
                arena.alloc(node).expect("Allocation in the arena failed");
            }
            black_box(arena.len());
        })
    });
}

fn bench_validate_only(c: &mut Criterion) {
    let big = past!({
        "root": {
            "level1": {
                "level2": {
                    "level3": {
                        "key": "value",
                        "array": [1, 2, 3, 4, 5],
                        "nested": {
                            "a": 1,
                            "b": 2,
                            "c": {
                                "deep": {
                                    "deeper": {
                                        "deepest": {
                                            "final_key": "final_value",
                                            "numbers": [10, 20, 30, 40, 50],
                                            "more_nested": {
                                                "x": 100,
                                                "y": 200
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    let limits = ast_core::ValidateLimits::default();

    c.bench_function("validate_only_big", |b| {
        b.iter(|| {
            black_box(big.validate_with(&limits)).ok();
        })
    });
}

fn bench_validate_code_ast(c: &mut Criterion) {
    let code_ast = past!({
        "type": "module",
        "name": "math",
        "functions": [
            {
                "type": "function",
                "name": "add",
                "params": [
                    { "name": "a", "type": "int" },
                    { "name": "b", "type": "int" }
                ],
                "body": {
                    "type": "return",
                    "value": {
                        "type": "binary_expr",
                        "operator": "+",
                        "left": { "type": "var", "name": "a" },
                        "right": { "type": "var", "name": "b" }
                    }
                }
            },
            {
                "type": "function",
                "name": "multiply",
                "params": [
                    { "name": "x", "type": "int" },
                    { "name": "y", "type": "int" }
                ],
                "body": {
                    "type": "return",
                    "value": {
                        "type": "binary_expr",
                        "operator": "*",
                        "left": { "type": "var", "name": "x" },
                        "right": { "type": "var", "name": "y" }
                    }
                }
            }
        ]
    });
    let limits = ast_core::ValidateLimits::default();

    c.bench_function("validate_code_ast", |b| {
        b.iter(|| {
            let _ = black_box(code_ast.validate_with(&limits));
        })
    });
}

fn bench_validate_html_ast(c: &mut Criterion) {
    let html_ast = past!({
        "type": "element",
        "tag": "html",
        "children": [
            {
                "type": "element",
                "tag": "head",
                "children": [
                    {
                        "type": "element",
                        "tag": "title",
                        "children": [
                            { "type": "text", "value": "Page Title" }
                        ]
                    }
                ]
            },
            {
                "type": "element",
                "tag": "body",
                "attributes": {
                    "class": "main-content"
                },
                "children": [
                    {
                        "type": "element",
                        "tag": "h1",
                        "children": [
                            { "type": "text", "value": "Welcome to the site!" }
                        ]
                    },
                    {
                        "type": "element",
                        "tag": "p",
                        "children": [
                            { "type": "text", "value": "This is a paragraph with more content." }
                        ]
                    },
                    {
                        "type": "element",
                        "tag": "ul",
                        "children": [
                            {
                                "type": "element",
                                "tag": "li",
                                "children": [
                                    { "type": "text", "value": "Item 1" }
                                ]
                            },
                            {
                                "type": "element",
                                "tag": "li",
                                "children": [
                                    { "type": "text", "value": "Item 2" }
                                ]
                            }
                        ]
                    }
                ]
            }
        ]
    });
    let limits = ast_core::ValidateLimits::default();

    c.bench_function("validate_html_ast", |b| {
        b.iter(|| {
            let _ = black_box(html_ast.validate_with(&limits));
        })
    });
}

fn bench_rebuild_and_validate_html(c: &mut Criterion) {
    let limits = ast_core::ValidateLimits::default();
    c.bench_function("rebuild_and_validate_html", |b| {
        b.iter(|| {
            let node = past!({
                "type": "element",
                "tag": "html",
                "children": [
                    { "type": "text", "value": "hello" },
                    { "type": "element", "tag": "div", "children": [ { "type": "text", "value": "x" } ] }
                ]
            });

            if let Err(e) = node.validate_with(&limits) {
                black_box(e);
            } else {
                black_box(());
            }
        })
    });
}

fn bench_build_only_no_drop(c: &mut Criterion) {
    let mut arena = SlotArena::with_capacity(1000);

    c.bench_function("build_only_no_drop_past", |b| {
        b.iter(|| {
            arena.drain().for_each(drop);

            for _ in 0..10 {
                for i in 0..100u32 {
                    let node = past!({
                        id: (i),
                        name: ("x"),
                        ok: true,
                        value: (i as i64),
                        nested: { a: 1, b: 2 }
                    });
                    arena.alloc(node).expect("Allocation in the arena failed");
                }
            }
            black_box(arena.len());
        })
    });
}

fn bench_drop_only_big(c: &mut Criterion) {
    let big = past!({
        "type": "element",
        "tag": "html",
        "children": [
            { "type": "text", "value": "hello" },
            { "type": "element", "tag": "div", "children": [ { "type": "text", "value": "x" } ] }
        ]
    });

    c.bench_function("drop_only_big_clone", |b| {
        b.iter(|| {
            let tmp = black_box(big.clone());
            drop(tmp);
        })
    });
}

criterion_group!(
    benches,
    bench_past_object_1000,
    bench_builder_object_1000,
    bench_validate_only,
    bench_validate_code_ast,
    bench_validate_html_ast,
    bench_rebuild_and_validate_html,
    bench_build_only_no_drop,
    bench_drop_only_big
);
criterion_main!(benches);
