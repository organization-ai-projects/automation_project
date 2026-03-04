use crate::diagnostics::backend_error::BackendError;
use crate::protocol::response::Response;
use std::io::Write;

pub struct JsonCodec;

impl JsonCodec {
    pub fn new() -> Self {
        Self
    }

    pub fn decode<T: serde::de::DeserializeOwned>(&self, line: &str) -> Result<T, BackendError> {
        common_json::from_str(line).map_err(|error| BackendError::Decode(error.to_string()))
    }

    pub fn write_line(&self, mut writer: impl Write, value: &Response) -> Result<(), BackendError> {
        let json = common_json::to_string(value)
            .map_err(|error| BackendError::Encode(error.to_string()))?;
        writeln!(writer, "{json}").map_err(|error| BackendError::Io(error.to_string()))
    }
}
