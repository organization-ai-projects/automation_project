// projects/products/unstable/hospital_tycoon/backend/src/io/json_codec.rs
use crate::diagnostics::app_error::AppError;
use crate::protocol::message::Message;
use crate::protocol::response::Response;

pub struct JsonCodec;

impl JsonCodec {
    pub fn decode_message(line: &str) -> Result<Message, AppError> {
        common_json::from_str(line).map_err(|e| AppError::Protocol(e.to_string()))
    }

    pub fn encode_response(resp: &Response) -> Result<String, AppError> {
        common_json::to_string(resp).map_err(|e| AppError::Protocol(e.to_string()))
    }
}
