mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("meta_determinism_guard ui starting");

    let args: Vec<String> = std::env::args().collect();
    public_api::run_cli(&args)
}
