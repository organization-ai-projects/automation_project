// projects/libraries/common_json/src/deserialization/tests/json_deserializable.rs
#[cfg(test)]
mod tests {
    use crate::deserialization::{from_json, from_str};
    use crate::json::Json;

    #[test]
    fn test_from_json() {
        let json = Json::Object(Default::default());
        let result: Result<Json, _> = from_json(&json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_str() {
        let json_str = "{\"key\":\"value\"}";
        let result: Result<Json, _> = from_str(json_str);
        assert!(result.is_ok());
    }
}
