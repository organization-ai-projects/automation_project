use crate::map::catalog::map_json_for_id;

#[test]
fn map_catalog_resolves_known_map_id() {
    let json = map_json_for_id("tiny_triangle").expect("known map id should resolve");
    assert!(json.contains("\"tiny_triangle\""));
}

#[test]
fn map_catalog_rejects_unknown_map_id() {
    assert!(map_json_for_id("unknown").is_none());
}
