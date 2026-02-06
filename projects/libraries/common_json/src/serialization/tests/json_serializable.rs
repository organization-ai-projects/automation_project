// projects/libraries/common_json/src/serialization/tests/json_serializable.rs
#[cfg(test)]
mod tests {
    use crate::JsonSerializable;
    use crate::json::Json;
    use crate::json_error::JsonError;
    use crate::json_error_code::JsonErrorCode;
    use std::error::Error;
    use std::io::Write;

    type TestResult = Result<(), Box<dyn Error>>;

    struct TestStruct {
        field: String,
    }

    impl JsonSerializable for TestStruct {
        fn to_json(&self) -> Result<Json, JsonError> {
            Ok(Json::String(self.field.clone()))
        }

        fn to_json_string(&self) -> Result<String, JsonError> {
            Ok(self.field.clone())
        }

        fn to_json_string_pretty(&self) -> Result<String, JsonError> {
            Ok(format!("{{\n  \"field\": \"{}\"\n}}", self.field))
        }

        fn to_json_bytes(&self) -> Result<Vec<u8>, JsonError> {
            Ok(self.field.as_bytes().to_vec())
        }

        fn write_json<W: Write>(&self, mut writer: W) -> Result<(), JsonError> {
            writer
                .write_all(self.field.as_bytes())
                .map_err(|_| JsonError {
                    code: JsonErrorCode::Io,
                    context: None,
                    source: None,
                })
        }

        fn write_json_pretty<W: Write>(&self, mut writer: W) -> Result<(), JsonError> {
            let pretty = format!("{{\n  \"field\": \"{}\"\n}}", self.field);
            writer.write_all(pretty.as_bytes()).map_err(|_| JsonError {
                code: JsonErrorCode::Io,
                context: None,
                source: None,
            })
        }
    }

    #[test]
    fn test_json_serializable() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let json = test.to_json()?;
        assert_eq!(json, Json::String("test_value".to_string()));
        Ok(())
    }

    #[test]
    fn test_to_json_string() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let json_string = test.to_json_string()?;
        assert_eq!(json_string, "test_value".to_string());
        Ok(())
    }

    #[test]
    fn test_to_json_string_pretty() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let json_string_pretty = test.to_json_string_pretty()?;
        assert_eq!(
            json_string_pretty,
            "{\n  \"field\": \"test_value\"\n}".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_to_json_bytes() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let json_bytes = test.to_json_bytes()?;
        assert_eq!(json_bytes, b"test_value".to_vec());
        Ok(())
    }

    #[test]
    fn test_write_json() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let mut buffer = Vec::new();
        test.write_json(&mut buffer)?;
        assert_eq!(buffer, b"test_value".to_vec());
        Ok(())
    }

    #[test]
    fn test_write_json_pretty() -> TestResult {
        let test = TestStruct {
            field: "test_value".to_string(),
        };
        let mut buffer = Vec::new();
        test.write_json_pretty(&mut buffer)?;
        assert_eq!(buffer, b"{\n  \"field\": \"test_value\"\n}".to_vec());
        Ok(())
    }
}
