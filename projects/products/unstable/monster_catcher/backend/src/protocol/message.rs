use crate::protocol::response::Response;
use anyhow::Result;
use std::io::Write;

pub fn write_response_stdout(response: &Response) -> Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    let line = common_json::to_string(response)?;
    writeln!(&mut lock, "{line}")?;
    Ok(())
}
