//! projects/products/varina/backend/src/app.rs
use std::io::{self, Read};

use anyhow::Context;
use common_json::{from_json_str, to_json_string_pretty};
use protocol::Command;

use crate::router::handle_command;

/// Reads a command from stdin, routes it, and writes the response to stdout.
pub fn run_backend() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let cmd: Command = from_json_str(&input).context("Invalid JSON input for Command")?;

    let response = handle_command(cmd);
    println!("{}", to_json_string_pretty(&response)?);
    Ok(())
}
