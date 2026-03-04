// projects/products/unstable/protocol_builder/backend/src/main.rs
mod diagnostics;
mod generate;
mod io;
mod output;
mod parse;
mod protocol;
mod public_api;
mod schema;

use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("protocol_builder backend starting");
    protocol::server::run()
}
