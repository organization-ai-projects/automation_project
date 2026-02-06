// Integration tests for ast_macros

use ast_core::{AstBuilder, AstKey, AstKind, AstNode};
use ast_macros::{build_array, build_object, key, value};

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
    
    match &arr.kind {
        AstKind::Array(elements) => {
            assert_eq!(elements.len(), 3);
        }
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_build_object_macro() {
    let obj = build_object!({
        name: "test",
        count: 42
    });
    
    match &obj.kind {
        AstKind::Object(fields) => {
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected object"),
    }
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
