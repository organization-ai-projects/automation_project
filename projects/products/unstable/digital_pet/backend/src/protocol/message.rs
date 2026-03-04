// projects/products/unstable/digital_pet/backend/src/protocol/message.rs
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<u64>,
    pub request: Request,
}

pub fn write_response_stdout(
    response: &Response,
) -> Result<(), crate::diagnostics::app_error::AppError> {
    let encoded = crate::io::json_codec::JsonCodec::encode_response(response)?;
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    writeln!(out, "{encoded}")
        .map_err(|error| crate::diagnostics::app_error::AppError::Io(error.to_string()))?;
    out.flush()
        .map_err(|error| crate::diagnostics::app_error::AppError::Io(error.to_string()))
}
