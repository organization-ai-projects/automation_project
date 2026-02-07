// projects/libraries/common_json/src/tests/json_object_builder.rs
use super::test_helpers::assert_json_object;
use crate::json_object_builder::JsonObjectBuilder;

#[test]
fn test_json_object_builder_add_field() {
    let builder = JsonObjectBuilder::new()
        .field("key1", "value1")
        .field("key2", "value2");
    let object = builder.build();
    assert_json_object(&object);
}
