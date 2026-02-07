// projects/libraries/common_json/src/tests/json_array_builder.rs
use super::test_helpers::assert_json_array;
use crate::Json;
use crate::json_array_builder::JsonArrayBuilder;

#[test]
fn test_json_array_builder() {
    let mut builder = JsonArrayBuilder::new();
    builder = builder.element("value1");
    builder = builder.element("value2");
    let array = builder.build();

    assert_json_array(&array);
    let Json::Array(arr) = array else {
        panic!("Result is not a JSON array");
    };
    assert_eq!(arr.len(), 2);
    assert!(arr.contains(&Json::from("value1")));
    assert!(arr.contains(&Json::from("value2")));
}
