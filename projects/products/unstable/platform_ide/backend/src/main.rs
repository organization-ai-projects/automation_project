// projects/products/unstable/platform_ide/backend/src/main.rs
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
        "Platform IDE backend starting"
    );
    tracing::info!("Platform IDE backend initialised.");

    Ok(())
}
