use crate::report::run_hash::{canonical_json_string, compute_run_hash_from_json};

#[test]
fn canonical_json_sorts_object_keys() {
    let json_a: common_json::Json = common_json::from_str(r#"{"b":2,"a":1}"#).expect("json a");
    let json_b: common_json::Json = common_json::from_str(r#"{"a":1,"b":2}"#).expect("json b");

    let canonical_a = canonical_json_string(&json_a);
    let canonical_b = canonical_json_string(&json_b);
    assert_eq!(canonical_a, canonical_b);

    let hash_a = compute_run_hash_from_json(&canonical_a);
    let hash_b = compute_run_hash_from_json(&canonical_b);
    assert_eq!(hash_a, hash_b);
}
