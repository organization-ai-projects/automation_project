use crate::diagnostics::error::BackendError;
use crate::protocol::request::Request;

pub struct JsonCodec;

impl JsonCodec {
    pub fn parse_request(line: &str) -> Result<Request, BackendError> {
        common_json::from_json_str(line).map_err(|e| BackendError::Codec(e.to_string()))
    }
}
