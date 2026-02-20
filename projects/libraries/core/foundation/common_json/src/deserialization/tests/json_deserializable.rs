// projects/libraries/common_json/src/deserialization/tests/json_deserializable.rs
#[cfg(test)]
mod tests {
    use crate::deserialization::{from_json, from_str};
    use crate::json::Json;
    type TestResult = crate::JsonResult<()>;

    #[test]
    fn test_from_json() -> TestResult {
        let json = Json::Object(Default::default());
        let _result: Json = from_json(&json)?;
        Ok(())
    }

    #[test]
    fn test_from_str() -> TestResult {
        let json_str = "{\"key\":\"value\"}";
        let _result: Json = from_str(json_str)?;
        Ok(())
    }
}
