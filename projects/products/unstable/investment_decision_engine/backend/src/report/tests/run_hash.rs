use crate::report::RunHash;

#[test]
fn same_input_same_hash() {
    let h1 = RunHash::compute("test input");
    let h2 = RunHash::compute("test input");
    assert_eq!(h1, h2);
}

#[test]
fn different_input_different_hash() {
    let h1 = RunHash::compute("input a");
    let h2 = RunHash::compute("input b");
    assert_ne!(h1, h2);
}

#[test]
fn hash_is_hex_string() {
    let h = RunHash::compute("test");
    assert_eq!(h.0.len(), 64);
    assert!(h.0.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn serialization_roundtrip() {
    let h = RunHash::compute("test");
    let json = common_json::to_json_string(&h).unwrap();
    let restored: RunHash = common_json::from_str(&json).unwrap();
    assert_eq!(h, restored);
}
