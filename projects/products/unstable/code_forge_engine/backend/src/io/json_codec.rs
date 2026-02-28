// projects/products/unstable/code_forge_engine/backend/src/io/json_codec.rs
use std::io::Write;
use crate::diagnostics::error::ForgeError;
use crate::protocol::response::Response;

pub struct JsonCodec;

impl JsonCodec {
    pub fn new() -> Self {
        Self
    }

    pub fn decode<T: serde::de::DeserializeOwned>(&self, line: &str) -> Result<T, ForgeError> {
        serde_json::from_str(line).map_err(|e| ForgeError::Decode(e.to_string()))
    }

    pub fn write_line(&self, mut writer: impl Write, value: &Response) -> Result<(), ForgeError> {
        let json = serde_json::to_string(value).map_err(|e| ForgeError::Encode(e.to_string()))?;
        writeln!(writer, "{json}").map_err(|e| ForgeError::Io(e.to_string()))
    }
}
