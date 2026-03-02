mod cli;
mod diagnostics;
mod public_api;
mod render;
mod transport;

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    let exit_code = match public_api::run_cli(&args) {
        Ok(code) => code,
        Err(err) => {
            let wrapped = diagnostics::error::Error::Transport(err.to_string());
            eprintln!("{wrapped}");
            2
        }
    };
    std::process::exit(exit_code);
}
