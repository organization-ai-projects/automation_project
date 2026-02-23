// projects/products/unstable/platform_ide/src/main.rs
mod app;
mod auth;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = app::IdeConfig::from_env();

    tracing::info!(
        platform_url = %config.platform_url,
        "Platform IDE starting"
    );

    // In production the IDE binary integrates a full interactive TUI or web
    // frontend. For the MVP the binary exits cleanly after printing the
    // configuration, demonstrating that the IDE initialises correctly.
    tracing::info!("Platform IDE initialised. Connect via PLATFORM_IDE_URL.");

    Ok(())
}
