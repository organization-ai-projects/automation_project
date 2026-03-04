use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::io::json_codec;
use crate::protocol::response::Response;

/// A generic envelope for framing IPC messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub id: u64,
    pub payload: T,
}

pub fn write_response_stdout(response: &Response) -> std::io::Result<()> {
    let line = json_codec::encode_response(response).map_err(std::io::Error::other)?;
    write_stdout_line(&line)
}

pub fn write_stdout_line(line: &str) -> std::io::Result<()> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{line}")
}

pub fn write_stderr_line(line: &str) -> std::io::Result<()> {
    let mut err = std::io::stderr().lock();
    writeln!(err, "{line}")
}
