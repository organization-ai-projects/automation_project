// projects/libraries/common_json/src/serialization/tests/json_map_serializer.rs
#[cfg(test)]
mod tests {
    use crate::Json;
    use crate::serialization::json_map_serializer::JsonMapSerializer;
    use serde::ser::SerializeMap;
    use serde::ser::SerializeStruct;
    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn test_json_map_serializer() -> TestResult {
        let mut serializer = JsonMapSerializer::with_capacity(2);
        serializer.serialize_key("key1")?;
        serializer.serialize_value("value1")?;
        serializer.serialize_key("key2")?;
        serializer.serialize_value("value2")?;
        Ok(())
    }

    #[test]
    fn test_json_map_serializer_end() -> TestResult {
        let mut serializer = JsonMapSerializer::with_capacity(2);
        serializer.serialize_key("key1")?;
        serializer.serialize_value("value1")?;
        serializer.serialize_key("key2")?;
        serializer.serialize_value("value2")?;
        let result = SerializeMap::end(serializer)?;

        let Json::Object(map) = result else {
            panic!("Expected Json::Object");
        };
        assert_eq!(map.get("key1"), Some(&Json::String("value1".to_string())));
        assert_eq!(map.get("key2"), Some(&Json::String("value2".to_string())));
        Ok(())
    }

    #[test]
    fn test_json_map_serializer_serialize_field() -> TestResult {
        let mut serializer = JsonMapSerializer::with_capacity(1);
        serializer.serialize_field("field1", "value1")?;
        let result = SerializeStruct::end(serializer)?;

        let Json::Object(map) = result else {
            panic!("Expected Json::Object");
        };
        assert_eq!(map.get("field1"), Some(&Json::String("value1".to_string())));
        Ok(())
    }
}
