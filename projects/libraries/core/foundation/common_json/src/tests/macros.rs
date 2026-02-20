// projects/libraries/common_json/src/tests/macros.rs
use crate::Json;
use crate::{json_array, json_object, pjson};

#[test]
fn test_pjson_complex_expressions_without_parens() {
    let path = std::path::Path::new("/tmp/test");

    let obj = pjson!({
        "simple": 42,
        "method_call": path.to_string_lossy().to_string(),
        "chained": "hello".to_uppercase(),
        "arithmetic": 1 + 2 * 3
    });

    assert!(obj.is_object());
    let map = obj.as_object().expect("object");
    assert_eq!(map.get("simple"), Some(&Json::from(42)));
    assert_eq!(
        map.get("method_call"),
        Some(&Json::from(path.to_string_lossy().to_string()))
    );
    assert_eq!(map.get("chained"), Some(&Json::from("HELLO")));
    assert_eq!(map.get("arithmetic"), Some(&Json::from(7)));
}

#[test]
fn test_pjson_nested_objects_and_arrays() {
    let name = "Alice";
    let age = 30;

    let obj = pjson!({
        "user": {
            "name": name,
            "age": age,
            "computed": age * 2
        },
        "tags": ["admin", "user"],
        "scores": [1, 2, 3]
    });

    assert!(obj.is_object());
}

#[test]
fn test_pjson_all_types() {
    let obj = pjson!({
        "null_val": null,
        "bool_true": true,
        "bool_false": false,
        "negative": -42,
        "float": std::f64::consts::PI,
        "string": "hello",
        "array": [1, 2, 3],
        "nested": { "a": 1 }
    });

    let map = obj.as_object().expect("Object conversion failed");
    assert_eq!(map.get("null_val"), Some(&Json::Null));
    assert_eq!(map.get("bool_true"), Some(&Json::Bool(true)));
    assert_eq!(map.get("bool_false"), Some(&Json::Bool(false)));
}

#[test]
fn test_pjson_variables_direct() {
    let x = 42;
    let s = "test";
    let v = vec![1, 2, 3];

    assert_eq!(pjson!(x), Json::from(42));
    assert_eq!(pjson!(s), Json::from("test"));

    let arr = pjson!(v);
    assert!(arr.is_array());
}

#[test]
fn test_json_array_macro() {
    let arr = json_array![1, 2, 3];
    assert!(arr.is_array());
    assert_eq!(arr.as_array().expect("Array conversion failed").len(), 3);

    let mixed = json_array!["hello", 42, true];
    assert_eq!(mixed.as_array().expect("Array conversion failed").len(), 3);
}

#[test]
fn test_json_object_macro() {
    let obj = json_object! {
        "name" => "Alice",
        "age" => 30,
    };

    let map = obj.as_object().expect("Object conversion failed");
    assert_eq!(map.get("name"), Some(&Json::from("Alice")));
    assert_eq!(map.get("age"), Some(&Json::from(30)));
}

#[test]
fn test_pjson_ident_keys() {
    let obj = pjson!({
        name: "Alice",
        age: 30,
        active: true
    });

    let map = obj.as_object().expect("Object conversion failed");
    assert_eq!(map.get("name"), Some(&Json::from("Alice")));
    assert_eq!(map.get("age"), Some(&Json::from(30)));
    assert_eq!(map.get("active"), Some(&Json::Bool(true)));
}

#[test]
fn test_pjson_dynamic_key() {
    let key_name = "dynamic_key";
    let obj = pjson!({
        (key_name): 42
    });

    let map = obj.as_object().expect("Object conversion failed");
    assert_eq!(map.get("dynamic_key"), Some(&Json::from(42)));
}
