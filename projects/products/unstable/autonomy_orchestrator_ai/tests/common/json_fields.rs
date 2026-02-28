use common_json::{Json, JsonAccess};

pub fn json_field_str<'a>(value: &'a Json, key: &str) -> Option<&'a str> {
    value.get_field(key).and_then(|v| v.as_str_strict()).ok()
}

pub fn json_field_u64(value: &Json, key: &str) -> Option<u64> {
    value.get_field(key).and_then(|v| v.as_u64_strict()).ok()
}

pub fn json_field_array<'a>(value: &'a Json, key: &str) -> Option<&'a Vec<Json>> {
    value.get_field(key).and_then(|v| v.as_array_strict()).ok()
}
