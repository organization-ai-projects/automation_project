//! projects/products/stable/platform_versioning/backend/src/main.rs
mod app;
mod app_config;
mod auth;
mod checkouts;
mod diffs;
mod errors;
mod history;
mod http;
mod ids;
mod indexes;
mod issues;
mod merges;
mod nonce;
mod objects;
mod pipeline;
mod refs_store;
mod repos;
mod routes;
mod routes_types;
mod slices;
mod sync;
mod verify;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = app_config::AppConfig::from_env()?;
    app::App::new(config).run().await?;
    Ok(())
}
