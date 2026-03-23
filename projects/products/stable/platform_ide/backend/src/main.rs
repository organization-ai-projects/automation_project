//! projects/products/stable/platform_ide/backend/src/main.rs
mod app;
mod auth;
mod bootstrap;
mod changes;
mod client;
mod diff;
mod editor;
mod errors;
mod issues;
mod offline;
mod slices;
mod verification;

#[cfg(test)]
mod tests;

use anyhow::Context;
use app::IdeConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = IdeConfig::from_env();

    tracing::info!(
        platform_url = %config.platform_url,
        "Platform IDE backend starting"
    );

    bootstrap::run_local_bootstrap(&config).context("local bootstrap failed")?;
    bootstrap::run_remote_bootstrap(&config).await;

    tracing::info!("Platform IDE backend initialised");

    Ok(())
}
