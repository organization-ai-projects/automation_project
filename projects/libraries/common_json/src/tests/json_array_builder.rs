// projects/libraries/common_json/src/tests/json_array_builder.rs
use crate::Json;
use crate::json_array_builder::JsonArrayBuilder;

#[test]
fn test_json_array_builder() {
    let mut builder = JsonArrayBuilder::new();
    builder = builder.element("value1");
    builder = builder.element("value2");
    let array = builder.build();
    if let Json::Array(arr) = array {
        assert_eq!(arr.len(), 2);
        assert!(arr.contains(&Json::from("value1")));
        assert!(arr.contains(&Json::from("value2")));
    } else {
        panic!("Le résultat n'est pas un tableau JSON");
    }
}
