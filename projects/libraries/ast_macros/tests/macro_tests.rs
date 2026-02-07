// Integration tests for ast_macros

use ast_core::{AstKey, AstKind, AstNode, ValidateLimits};
use ast_macros::{apply_cfg, build_array, build_object, key, validate_preset, value};

// Helper functions for common assertions
fn assert_array_len(node: &AstNode, expected_len: usize) {
    let AstKind::Array(elements) = &node.kind else {
        panic!("Expected array, got {:?}", node.kind);
    };
    assert_eq!(elements.len(), expected_len);
}

fn assert_object_len(node: &AstNode, expected_len: usize) {
    let AstKind::Object(fields) = &node.kind else {
        panic!("Expected object, got {:?}", node.kind);
    };
    assert_eq!(fields.len(), expected_len);
}

#[test]
fn test_key_macro() {
    // Test identifier key
    let k1 = key!(field);
    assert!(matches!(k1, AstKey::Ident(_)));

    // Test string literal key
    let k2 = key!("field");
    assert!(matches!(k2, AstKey::String(_)));

    // Test expression key
    let name = "dynamic";
    let k3 = key!((name));
    assert!(matches!(k3, AstKey::String(_)));
}

#[test]
fn test_value_macro() {
    // Test null
    let null_val = value!(null);
    assert!(matches!(null_val.kind, AstKind::Null));

    // Test bool
    let true_val = value!(true);
    assert!(matches!(true_val.kind, AstKind::Bool(true)));

    let false_val = value!(false);
    assert!(matches!(false_val.kind, AstKind::Bool(false)));

    // Test number
    let num = value!(42);
    assert!(matches!(num.kind, AstKind::Number(_)));

    // Test negative number
    let neg = value!(-42);
    assert!(matches!(neg.kind, AstKind::Number(_)));

    // Test array
    let arr = value!([1, 2, 3]);
    assert!(matches!(arr.kind, AstKind::Array(_)));

    // Test object
    let obj = value!({ key: "value" });
    assert!(matches!(obj.kind, AstKind::Object(_)));
}

#[test]
fn test_build_array_macro() {
    let arr = build_array!([1, 2, 3]);
    assert_array_len(&arr, 3);
}

#[test]
fn test_build_object_macro() {
    let obj = build_object!({
        name: "test",
        count: 42
    });
    assert_object_len(&obj, 2);
}

#[test]
fn test_nested_structures() {
    let nested = value!({
        data: {
            items: [1, 2, 3],
            meta: {
                count: 3
            }
        }
    });

    assert!(matches!(nested.kind, AstKind::Object(_)));
}

#[test]
fn test_validate_preset_macro() {
    // Test default preset
    let node = value!({ small: true });
    let result = validate_preset!(node, default);
    assert!(result.is_ok());

    // Test strict preset
    let node = value!({ data: [1, 2, 3] });
    let result = validate_preset!(node, strict);
    assert!(result.is_ok());

    // Test unbounded preset
    let large_node = value!({
        items: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    });
    let result = validate_preset!(large_node, unbounded);
    assert!(result.is_ok());
}

#[test]
fn test_apply_cfg_macro() {
    // Test apply_cfg with max_depth
    let mut limits = ValidateLimits::default();
    apply_cfg!(limits, max_depth: 5);
    assert_eq!(limits.max_depth, 5);

    // Test apply_cfg with max_size
    let mut limits = ValidateLimits::default();
    apply_cfg!(limits, max_size: 50);
    assert_eq!(limits.max_size, 50);

    // Test apply_cfg with both settings
    let mut limits = ValidateLimits::default();
    apply_cfg!(limits, max_depth: 10, max_size: 100);
    assert_eq!(limits.max_depth, 10);
    assert_eq!(limits.max_size, 100);
}
