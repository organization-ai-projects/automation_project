// projects/libraries/ast_core/src/tests/ast_macro_tests.rs
// Tests for the `past!` macro

use crate::{AstBuilder, AstErrorKind, AstKey, AstNode, ValidateLimits, past};

#[test]
fn test_deeply_nested_structure() {
        let depth = 1000;
        let mut nested = past!(null);

        for _ in 0..depth {
            nested = past!({ "key": nested });
        }

        assert!(nested.is_object());
    }

#[test]
fn test_very_deeply_nested_structure() {
        let depth = 10_000; // Augmentation de la profondeur
        let mut nested = past!(null);

        for _ in 0..depth {
            nested = past!({ "key": nested });
        }

        let limits = ValidateLimits::unbounded();
        assert!(nested.validate_iterative(&limits).is_ok());
    }

#[test]
fn validate_cuts_before_stack_overflow() {
        let mut nested = past!(null);
        for _ in 0..10_000 {
            nested = past!({ "key": nested });
        }

        let limits = ValidateLimits {
            max_depth: 256,
            max_size: 100_000,
        };
        let err = nested.validate_iterative(&limits).unwrap_err();
        assert!(matches!(err.kind, AstErrorKind::MaxDepth { .. }));
    }

#[test]
fn validate_large_wide_tree() {
    // Stress test: validate a very wide tree with many keys to ensure
    // the validation system can handle large structures efficiently
    let mut fields = Vec::new();
    for i in 0..10_000 {
        fields.push(((format!("k{i}")), past!(i as i64)));
    }

    let node = AstBuilder::object::<AstKey, Vec<(AstKey, AstNode)>>(
        fields.into_iter().map(|(k, v)| (k.into(), v)).collect(),
    );

    let limits = ValidateLimits {
        max_depth: 32,
        max_size: 20_000,
    };
    node.validate_with(&limits).expect("validate wide tree");
}

#[test]
fn drop_deep_tree_on_small_stack() {
    // Stress test: ensure deeply nested structures can be dropped
    // safely on limited stack size (tests iterative drop implementation)
    let depth = 10_000;
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(move || {
            let mut nested = past!(null);
            for _ in 0..depth {
                nested = past!({ "key": nested });
            }
            drop(nested);
        })
        .expect("spawn test thread");

    handle.join().expect("join test thread");
}
