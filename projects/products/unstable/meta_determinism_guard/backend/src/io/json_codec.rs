use crate::diagnostics::backend_error::BackendError;
use crate::protocol::response::Response;

pub fn encode_response(response: &Response) -> Result<String, BackendError> {
    common_json::to_string(response).map_err(BackendError::from)
}

pub fn parse_request(line: &str) -> Result<crate::protocol::request::Request, BackendError> {
    common_json::from_json_str(line).map_err(BackendError::from)
}
