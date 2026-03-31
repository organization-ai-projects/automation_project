use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::diagnostics::Error;

pub struct JsonCodec;

impl JsonCodec {
    pub fn encode<T: Serialize>(value: &T) -> Result<String, Error> {
        common_json::to_string_pretty(value).map_err(|e| Error::Serialization(e.to_string()))
    }

    pub fn decode<T: DeserializeOwned>(data: &str) -> Result<T, Error> {
        common_json::from_str(data).map_err(|e| Error::Deserialization(e.to_string()))
    }
}
