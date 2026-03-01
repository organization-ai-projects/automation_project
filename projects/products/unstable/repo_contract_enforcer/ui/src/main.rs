mod cli;
mod diagnostics;
mod public_api;
mod render;
mod transport;

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let _error_type_marker = std::mem::size_of::<diagnostics::error::Error>();
    let _error_value_marker = diagnostics::error::Error::Transport(String::new());
    let args: Vec<String> = std::env::args().collect();
    let exit_code = match public_api::run_cli(&args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            2
        }
    };
    std::process::exit(exit_code);
}
