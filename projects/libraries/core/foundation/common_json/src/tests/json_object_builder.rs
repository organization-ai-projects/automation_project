// projects/libraries/common_json/src/tests/json_object_builder.rs
use crate::Json;
use crate::json_object_builder::JsonObjectBuilder;

#[test]
fn test_json_object_builder_add_field() {
    let builder = JsonObjectBuilder::new()
        .field("key1", "value1")
        .field("key2", "value2");
    let object = builder.build();
    assert!(matches!(object, Json::Object(_)));
}
