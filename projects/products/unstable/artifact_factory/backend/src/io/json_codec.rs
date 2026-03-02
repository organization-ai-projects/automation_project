use crate::diagnostics::error::FactoryError;
use crate::protocol::{RequestMessage, ResponseMessage};

pub struct JsonCodec;

impl JsonCodec {
    pub fn decode_request(line: &str) -> Result<RequestMessage, FactoryError> {
        common_json::from_json_str(line).map_err(|e| FactoryError::Codec(e.to_string()))
    }

    pub fn encode_response(msg: &ResponseMessage) -> Result<String, FactoryError> {
        common_json::to_string(msg).map_err(|e| FactoryError::Codec(e.to_string()))
    }
}
