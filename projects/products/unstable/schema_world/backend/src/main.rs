mod canon;
mod diagnostics;
mod diff;
mod io;
mod migrate;
mod protocol;
mod reports;
mod schemas;
mod snapshots;
mod storage;
mod validate;

use crate::diagnostics::error::Error;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    tracing::info!("schema_world backend starting");
    protocol::message::run_server_loop().map_err(|e| Error::Io(e.to_string()))
}
