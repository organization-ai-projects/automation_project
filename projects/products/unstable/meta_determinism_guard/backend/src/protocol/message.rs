use std::io::Write;

use crate::io::json_codec;
use crate::protocol::response::Response;

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
