use crate::io::json_codec::JsonCodec;
use crate::protocol::response::Response;
use crate::protocol::response_message::ResponseMessage;
use std::io::Write;

pub fn write_response_stdout(id: u64, response: Response) -> std::io::Result<()> {
    let encoded = JsonCodec::encode_response(&ResponseMessage { id, response })
        .map_err(std::io::Error::other)?;
    write_stdout_line(&encoded)
}

pub fn write_stdout_line(line: &str) -> std::io::Result<()> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{line}")
}

pub fn write_stderr_line(line: &str) -> std::io::Result<()> {
    let mut err = std::io::stderr().lock();
    writeln!(err, "{line}")
}
