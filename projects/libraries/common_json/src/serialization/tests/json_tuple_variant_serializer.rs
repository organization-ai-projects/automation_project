#[cfg(test)]
mod tests {
    use crate::Json;
    use crate::serialization::json_tuple_variant_serializer::JsonTupleVariantSerializer;
    use crate::tests::test_helpers::TestResult;
    use serde::ser::SerializeTupleVariant;

    #[test]
    fn test_serialize_field() -> TestResult {
        let mut serializer = JsonTupleVariantSerializer {
            name: "TestVariant".to_string(),
            elements: Vec::new(),
        };

        serializer.serialize_field(&"value1")?;
        serializer.serialize_field(&"value2")?;

        assert_eq!(serializer.elements.len(), 2);
        assert_eq!(serializer.elements[0], Json::String("value1".to_string()));
        assert_eq!(serializer.elements[1], Json::String("value2".to_string()));
        Ok(())
    }

    #[test]
    fn test_end() -> TestResult {
        let serializer = JsonTupleVariantSerializer {
            name: "TestVariant".to_string(),
            elements: vec![
                Json::String("value1".to_string()),
                Json::String("value2".to_string()),
            ],
        };

        let result = serializer.end()?;
        
        let Json::Object(map) = result else {
            panic!("Expected Json::Object");
        };
        assert!(map.contains_key("TestVariant"));
        let Some(Json::Array(elements)) = map.get("TestVariant") else {
            panic!("Expected Json::Array");
        };
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], Json::String("value1".to_string()));
        assert_eq!(elements[1], Json::String("value2".to_string()));
        Ok(())
    }
}
