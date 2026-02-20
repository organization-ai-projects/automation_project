// projects/libraries/common_json/src/deserialization/tests/json_seq_access.rs
use crate::Json;
use crate::deserialization::json_seq_access::*;
use serde::de::SeqAccess;
use std::marker::PhantomData;

#[test]
fn test_json_seq_access_valid() {
    let data = [Json::Null, Json::Null];
    let mut seq = JsonSeqAccess::new(data.iter());
    let seed = PhantomData::<()>;
    assert!(seq.next_element_seed(seed).is_ok());
}

#[test]
fn test_json_seq_access_empty() {
    let data: Vec<Json> = Vec::new();
    let mut seq = JsonSeqAccess::new(data.iter());
    let seed = PhantomData::<()>;
    assert!(seq.next_element_seed(seed).is_ok());
}
