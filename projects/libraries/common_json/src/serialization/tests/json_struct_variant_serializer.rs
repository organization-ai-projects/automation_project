// projects/libraries/common_json/src/serialization/tests/json_struct_variant_serializer.rs
use crate::Json;
use crate::serialization::json_struct_variant_serializer::JsonStructVariantSerializer;
use crate::tests::test_helpers::TestResult;
use serde::ser::SerializeStructVariant;

#[test]
fn test_serialize_field() -> TestResult {
    let mut serializer = JsonStructVariantSerializer {
        name: "TestStructVariant".to_string(),
        map: Default::default(),
    };

    serializer.serialize_field("key1", &"value1")?;
    serializer.serialize_field("key2", &"value2")?;

    assert_eq!(serializer.map.len(), 2);
    assert_eq!(
        serializer.map.get("key1"),
        Some(&Json::String("value1".to_string()))
    );
    assert_eq!(
        serializer.map.get("key2"),
        Some(&Json::String("value2".to_string()))
    );
    Ok(())
}

#[test]
fn test_end() -> TestResult {
    let serializer = JsonStructVariantSerializer {
        name: "TestStructVariant".to_string(),
        map: [
            ("key1".to_string(), Json::String("value1".to_string())),
            ("key2".to_string(), Json::String("value2".to_string())),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let result = serializer.end()?;
    
    let Json::Object(wrapper) = result else {
        panic!("Expected Json::Object");
    };
    assert!(wrapper.contains_key("TestStructVariant"));
    let Some(Json::Object(map)) = wrapper.get("TestStructVariant") else {
        panic!("Expected Json::Object");
    };
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&Json::String("value1".to_string())));
    assert_eq!(map.get("key2"), Some(&Json::String("value2".to_string())));
    Ok(())
}
