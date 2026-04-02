use crate::canonical_json::CanonicalJson;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Sample {
    zebra: u32,
    alpha: String,
    middle: bool,
}

#[test]
fn canonical_json_is_deterministic() {
    let data = Sample {
        zebra: 42,
        alpha: "hello".to_string(),
        middle: true,
    };

    let json1 = CanonicalJson::to_string(&data).unwrap();
    let json2 = CanonicalJson::to_string(&data).unwrap();
    assert_eq!(json1, json2);
}

#[test]
fn canonical_json_sorts_keys() {
    let data = Sample {
        zebra: 42,
        alpha: "hello".to_string(),
        middle: true,
    };

    let json = CanonicalJson::to_string(&data).unwrap();
    let alpha_pos = json.find("\"alpha\"").unwrap();
    let middle_pos = json.find("\"middle\"").unwrap();
    let zebra_pos = json.find("\"zebra\"").unwrap();

    assert!(alpha_pos < middle_pos);
    assert!(middle_pos < zebra_pos);
}

#[test]
fn canonical_json_formats_numbers_consistently() {
    let mut map = HashMap::new();
    map.insert("integer", 100_i64);
    map.insert("zero", 0);

    let json1 = CanonicalJson::to_string(&map).unwrap();
    let json2 = CanonicalJson::to_string(&map).unwrap();
    assert_eq!(json1, json2);
    assert!(json1.contains("100"));
    assert!(json1.contains("0"));
}

#[test]
fn canonical_json_handles_empty_collections() {
    let empty_vec: Vec<String> = vec![];
    let json = CanonicalJson::to_string(&empty_vec).unwrap();
    assert_eq!(json, "[]");

    let empty_map: HashMap<String, String> = HashMap::new();
    let json = CanonicalJson::to_string(&empty_map).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn canonical_json_escapes_special_chars() {
    let mut map = HashMap::new();
    map.insert("text", "line1\nline2\ttab");

    let json = CanonicalJson::to_string(&map).unwrap();
    assert!(json.contains("\\n"));
    assert!(json.contains("\\t"));
}
