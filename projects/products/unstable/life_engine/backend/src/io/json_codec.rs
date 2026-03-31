use common_json::{JsonResult, from_str, to_json_string_pretty};
use serde::{Serialize, de::DeserializeOwned};

pub struct JsonCodec;

impl JsonCodec {
    pub fn encode<T: Serialize>(value: &T) -> JsonResult<String> {
        to_json_string_pretty(value)
    }

    pub fn decode<T: DeserializeOwned>(input: &str) -> JsonResult<T> {
        from_str(input)
    }
}
