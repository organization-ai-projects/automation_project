use crate::io::json_codec::JsonCodec;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Sample {
    value: u64,
}

#[test]
fn encode_decode_roundtrip() {
    let s = Sample { value: 42 };
    let encoded = JsonCodec::encode(&s).unwrap();
    let decoded: Sample = JsonCodec::decode(&encoded).unwrap();
    assert_eq!(decoded, s);
}
