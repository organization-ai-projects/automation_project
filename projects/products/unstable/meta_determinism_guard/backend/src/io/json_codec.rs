use crate::protocol::response::Response;
use anyhow::Result;
use std::io::Write;

pub fn write_response<W: Write>(mut writer: W, response: &Response) -> Result<()> {
    let json = serde_json::to_string(response)?;
    writeln!(writer, "{}", json)?;
    Ok(())
}

pub fn parse_request(line: &str) -> Result<crate::protocol::request::Request> {
    Ok(serde_json::from_str(line)?)
}
