// projects/products/stable/platform_versioning/backend/src/main.rs
mod app;
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
mod objects;
mod pipeline;
mod refs_store;
mod repos;
mod routes;
mod slices;
mod sync;
mod verify;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = app::AppConfig::from_env()?;
    app::run(config).await?;
    Ok(())
}
