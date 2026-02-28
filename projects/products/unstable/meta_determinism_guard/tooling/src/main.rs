mod diagnostics;
mod public_api;
mod rules;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("meta_determinism_guard tooling starting");

    let args: Vec<String> = std::env::args().collect();
    public_api::run_cli(&args)
}
