// projects/products/unstable/evolutionary_system_generator/backend/src/protocol/message.rs
use crate::io::json_codec::{decode, encode};
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use std::io::Write;

pub fn write_response(resp: &Response) -> String {
    match encode(resp) {
        Ok(line) => line,
        Err(err) => format!(
            r#"{{"type":"Error","message":"serialization error: {}"}}"#,
            err
        ),
    }
}

pub fn read_request(line: &str) -> Result<Request, String> {
    decode(line).map_err(|e| e.to_string())
}

pub fn write_stdout_line(line: &str) -> std::io::Result<()> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{line}")
}

pub fn write_stderr_line(line: &str) -> std::io::Result<()> {
    let mut err = std::io::stderr().lock();
    writeln!(err, "{line}")
}
