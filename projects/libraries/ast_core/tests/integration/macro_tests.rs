// projects/libraries/ast_core/tests/integration/macro_tests.rs
use ast_core::past;

#[test]
fn test_macro_build_object() {
    let node = past!({
        name: "test",
        value: 42,
        active: true
    });

    assert!(node.is_object());
    assert_eq!(
        node.get("name").expect("Missing 'name' key").as_string(),
        Some("test")
    );
    assert_eq!(
        node.get("value")
            .expect("Missing 'value' key")
            .as_number()
            .expect("'value' is not a number")
            .as_i64(),
        Some(42)
    );
    assert_eq!(
        node.get("active").expect("Missing 'active' key").as_bool(),
        Some(true)
    );
}

#[test]
fn test_macro_build_array() {
    let node = past!([1, 2, 3]);

    assert!(node.is_array());
    let arr = node.as_array().expect("Node is not an array");
    assert_eq!(arr.len(), 3);
    assert_eq!(
        arr[0]
            .as_number()
            .expect("Element 0 is not a number")
            .as_i64(),
        Some(1)
    );
    assert_eq!(
        arr[1]
            .as_number()
            .expect("Element 1 is not a number")
            .as_i64(),
        Some(2)
    );
    assert_eq!(
        arr[2]
            .as_number()
            .expect("Element 2 is not a number")
            .as_i64(),
        Some(3)
    );
}

#[test]
fn test_macro_validate_with_limits() {
    let node = past!({
        nested: {
            inner: true
        }
    });

    let limits = ast_core::ValidateLimits {
        max_depth: 1,
        max_size: 10,
    };

    let result = past!(node, validate: with: limits);
    assert!(result.is_err());
}

#[test]
fn test_macro_build_scalars() {
    let null_node = past!(null);
    assert!(null_node.is_null());

    let true_node = past!(true);
    assert_eq!(true_node.as_bool(), Some(true));

    let false_node = past!(false);
    assert_eq!(false_node.as_bool(), Some(false));

    let negative_node = past!(-42);
    assert_eq!(
        negative_node
            .as_number()
            .expect("Node is not a number")
            .as_i64(),
        Some(-42)
    );
}

#[test]
fn test_macro_validate_presets() {
    let node = past!({ small: true });

    let strict_result = past!(node, validate: preset: strict);
    assert!(strict_result.is_ok());

    let unbounded_result = past!(node, validate: preset: unbounded);
    assert!(unbounded_result.is_ok());

    let default_result = past!(node, validate: preset: default);
    assert!(default_result.is_ok());
}

#[test]
fn test_macro_nested_structures() {
    let node = past!({
        level1: {
            level2: {
                level3: {
                    key: "value"
                }
            }
        }
    });

    assert!(node.is_object());
    assert!(
        node.get("level1")
            .expect("Missing 'level1' key")
            .get("level2")
            .expect("Missing 'level2' key")
            .get("level3")
            .expect("Missing 'level3' key")
            .is_object()
    );
    assert_eq!(
        node.get("level1")
            .expect("Missing 'level1' key")
            .get("level2")
            .expect("Missing 'level2' key")
            .get("level3")
            .expect("Missing 'level3' key")
            .get("key")
            .expect("Missing 'key' key")
            .as_string(),
        Some("value")
    );
}

#[test]
fn test_macro_with_metadata() {
    let node = past!({
        data: "content"
    }, origin: ai("agent"), flags: ["generated", "test"], attrs: { "version": "1.0" });

    assert_eq!(node.meta.origin, Some(ast_core::Origin::Ai("agent")));
    assert!(node.meta.flags.contains("generated"));
    assert!(node.meta.flags.contains("test"));
    assert_eq!(node.meta.attrs.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_macro_empty_structures() {
    let empty_object = past!({});
    assert!(empty_object.is_object());
    assert!(
        empty_object
            .as_object()
            .expect("Empty object is not an object")
            .is_empty()
    );

    let empty_array = past!([]);
    assert!(empty_array.is_array());
    assert!(
        empty_array
            .as_array()
            .expect("Empty array is not an array")
            .is_empty()
    );
}

#[test]
fn test_macro_large_structure() {
    let node = past!({
        key0: 0,
        key1: 1,
        key2: 2,
        key3: 3,
        key4: 4,
        key5: 5,
        key6: 6,
        key7: 7,
        key8: 8,
        key9: 9
    });

    assert!(node.is_object());
    assert_eq!(node.as_object().expect("Node is not an object").len(), 10);
}

#[test]
fn test_macro_large_nested_structure() {
    let node = past!({
        level1: {
            level2: {
                level3: {
                    level4: {
                        level5: {
                            key: "deep_value"
                        }
                    }
                }
            }
        }
    });

    assert!(node.is_object());
    assert_eq!(
        node.get("level1")
            .expect("Missing 'level1' key")
            .get("level2")
            .expect("Missing 'level2' key")
            .get("level3")
            .expect("Missing 'level3' key")
            .get("level4")
            .expect("Missing 'level4' key")
            .get("level5")
            .expect("Missing 'level5' key")
            .get("key")
            .expect("Missing 'key' key")
            .as_string(),
        Some("deep_value")
    );
}

#[test]
fn test_macro_large_object() {
    let mut large_object = std::collections::HashMap::new();
    for i in 0..1000 {
        large_object.insert(format!("key{}", i), i);
    }

    let node = past!({
        key0: 0,
        key1: 1,
        key2: 2,
        key3: 3,
        key4: 4,
        key5: 5,
        key6: 6,
        key7: 7,
        key8: 8,
        key9: 9,
    });

    assert!(node.is_object());
    assert!(node.as_object().expect("Node is not an object").len() >= 10);
}
