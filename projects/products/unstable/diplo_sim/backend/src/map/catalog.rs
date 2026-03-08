pub fn available_map_ids() -> &'static [&'static str] {
    &["tiny_triangle"]
}

pub fn map_json_for_id(map_id: &str) -> Option<&'static str> {
    match map_id {
        "tiny_triangle" => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/maps/tiny_triangle_map.json"
        ))),
        _ => None,
    }
}
