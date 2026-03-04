// projects/products/unstable/code_forge_engine/backend/src/protocol/message.rs
use crate::diagnostics::backend_error::BackendError;
use crate::io::json_codec::JsonCodec;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub request: Request,
}

pub fn write_response_stdout(codec: &JsonCodec, response: &Response) -> Result<(), BackendError> {
    let stdout = std::io::stdout();
    codec.write_line(&stdout, response)
}
