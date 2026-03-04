use std::io::{self, Write};

use anyhow::Result;

pub(crate) fn write_usage(binary_name: &str) -> Result<()> {
    let mut stderr = io::stderr();
    stderr.write_all(format!("Usage: {binary_name} <repo_root> <runs_root>\n").as_bytes())?;
    Ok(())
}

pub(crate) fn write_json_line(json: &str) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(json.as_bytes())?;
    stdout.write_all(b"\n")?;
    Ok(())
}
