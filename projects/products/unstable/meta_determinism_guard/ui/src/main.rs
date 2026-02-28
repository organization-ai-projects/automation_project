mod public_api;
mod app;
mod transport;
mod screens;
mod widgets;
mod diagnostics;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("meta_determinism_guard ui starting");

    let args: Vec<String> = std::env::args().collect();
    public_api::run_cli(&args)
}
