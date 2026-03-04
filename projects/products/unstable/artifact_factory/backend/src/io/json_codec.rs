use crate::diagnostics::backend_error::BackendError;
use crate::protocol::{RequestMessage, ResponseMessage};

pub struct JsonCodec;

impl JsonCodec {
    pub fn decode_request(line: &str) -> Result<RequestMessage, BackendError> {
        common_json::from_json_str(line).map_err(|e| BackendError::Codec(e.to_string()))
    }

    pub fn encode_response(msg: &ResponseMessage) -> Result<String, BackendError> {
        common_json::to_string(msg).map_err(|e| BackendError::Codec(e.to_string()))
    }
}
