// projects/libraries/common_json/src/serialization/tests/json_seq_serializer.rs
use crate::Json;
use crate::serialization::json_seq_serializer::JsonSeqSerializer;
use crate::tests::test_helpers::TestResult;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct};

#[test]
fn test_serialize_element() -> TestResult {
    let mut serializer = JsonSeqSerializer {
        elements: Vec::new(),
    };

    SerializeSeq::serialize_element(&mut serializer, &"value1")?;
    SerializeSeq::serialize_element(&mut serializer, &"value2")?;

    assert_eq!(serializer.elements.len(), 2);
    assert_eq!(serializer.elements[0], Json::String("value1".to_string()));
    assert_eq!(serializer.elements[1], Json::String("value2".to_string()));
    Ok(())
}

#[test]
fn test_end() -> TestResult {
    let serializer = JsonSeqSerializer {
        elements: vec![
            Json::String("value1".to_string()),
            Json::String("value2".to_string()),
        ],
    };

    let result = SerializeSeq::end(serializer)?;

    let Json::Array(elements) = result else {
        panic!("Expected Json::Array");
    };
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0], Json::String("value1".to_string()));
    assert_eq!(elements[1], Json::String("value2".to_string()));
    Ok(())
}

#[test]
fn test_serialize_tuple() -> TestResult {
    let mut serializer = JsonSeqSerializer {
        elements: Vec::new(),
    };

    SerializeTuple::serialize_element(&mut serializer, &"value1")?;
    SerializeTuple::serialize_element(&mut serializer, &"value2")?;

    let result = SerializeTuple::end(serializer)?;
    if let Json::Array(elements) = result {
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], Json::String("value1".to_string()));
        assert_eq!(elements[1], Json::String("value2".to_string()));
    } else {
        panic!("Expected Json::Array");
    }
    Ok(())
}

#[test]
fn test_serialize_tuple_struct() -> TestResult {
    let mut serializer = JsonSeqSerializer {
        elements: Vec::new(),
    };

    SerializeTupleStruct::serialize_field(&mut serializer, &"value1")?;
    SerializeTupleStruct::serialize_field(&mut serializer, &"value2")?;

    let result = SerializeTupleStruct::end(serializer)?;
    if let Json::Array(elements) = result {
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], Json::String("value1".to_string()));
        assert_eq!(elements[1], Json::String("value2".to_string()));
    } else {
        panic!("Expected Json::Array");
    }
    Ok(())
}
