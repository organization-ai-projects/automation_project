// projects/products/unstable/platform_versioning/backend/src/main.rs
mod app;
mod auth;
mod checkout;
mod diff;
mod errors;
mod history;
mod http;
mod ids;
mod index;
mod merge;
mod objects;
mod pipeline;
mod refs_store;
mod repos;
mod routes;
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
