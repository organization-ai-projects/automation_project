mod config;
mod diagnostics;
mod io;
mod protocol;
mod public_api;
mod report;
mod rules;
mod scan;

use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut state = public_api::BackendState::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    loop {
        let mut line = String::new();
        let n = stdin.lock().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }

        let request: protocol::request::Request = match serde_json::from_str(line.trim()) {
            Ok(req) => req,
            Err(_) => {
                let response = protocol::response::Response;
                io::json_codec::JsonCodec::write_response(&stdout, &response)?;
                continue;
            }
        };

        let response = state.handle(request);
        io::json_codec::JsonCodec::write_response(&stdout, &response)?;
    }

    Ok(())
}
