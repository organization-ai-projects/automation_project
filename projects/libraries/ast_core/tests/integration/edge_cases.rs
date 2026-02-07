// projects/libraries/ast_core/tests/integration/edge_cases.rs
use crate::helpers::assert_error_matches;
use ast_core::{AstKey, AstKind, AstNode, Number, ValidateLimits};

#[test]
fn test_integration_empty_object() {
    let node = AstNode {
        kind: AstKind::Object(vec![]),
        meta: Default::default(),
    };

    let limits = ValidateLimits::default();
    let result = node.validate_with(&limits);
    assert!(result.is_ok());
}

#[test]
fn test_integration_empty_array() {
    let node = AstNode {
        kind: AstKind::Array(vec![]),
        meta: Default::default(),
    };

    let limits = ValidateLimits::default();
    let result = node.validate_with(&limits);
    assert!(result.is_ok());
}

#[test]
fn test_integration_large_structure() {
    let mut fields = vec![];
    for i in 0..100 {
        fields.push((
            AstKey::String(format!("key{}", i)),
            AstNode {
                kind: AstKind::Number(Number::Int(i as i64)),
                meta: Default::default(),
            },
        ));
    }

    let node = AstNode {
        kind: AstKind::Object(fields),
        meta: Default::default(),
    };

    let limits = ValidateLimits {
        max_depth: 10,
        max_size: 50,
    };

    let result = node.validate_with(&limits);
    assert_error_matches(result, "Exceeded maximum size for object: 50");
}
