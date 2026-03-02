use crate::protocol::response::Response;
use anyhow::Result;
use std::io::Write;

pub struct JsonCodec;

impl JsonCodec {
    pub fn write_response(stdout: &std::io::Stdout, response: &Response) -> Result<()> {
        let mut lock = stdout.lock();
        let line = common_json::to_string(response)?;
        writeln!(&mut lock, "{line}")?;
        Ok(())
    }
}
