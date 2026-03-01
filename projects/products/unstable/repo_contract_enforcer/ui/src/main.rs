mod cli;
mod diagnostics;
mod public_api;
mod render;
mod transport;

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    public_api::run_cli(&args)
}
