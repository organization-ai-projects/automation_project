mod public_api;
mod protocol;
mod scan;
mod canon;
mod stability;
mod io;
mod diagnostics;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("meta_determinism_guard backend starting");

    let mut state = public_api::BackendState::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    loop {
        let mut line = String::new();
        let n = {
            use std::io::BufRead;
            stdin.lock().read_line(&mut line)?
        };
        if n == 0 {
            break; // EOF
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let request: protocol::request::Request = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let resp = protocol::response::Response::Error { message: e.to_string() };
                io::json_codec::write_response(&stdout, &resp)?;
                continue;
            }
        };
        let response = state.handle(request);
        io::json_codec::write_response(&stdout, &response)?;
    }

    Ok(())
}
