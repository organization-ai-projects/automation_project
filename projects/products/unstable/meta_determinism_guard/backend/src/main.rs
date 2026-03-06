mod canon;
mod diagnostics;
mod io;
mod protocol;
mod public_api;
mod scan;
mod stability;
mod tooling;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("meta_determinism_guard backend starting");

    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        let exit_code = tooling::deterministic_cli::run_cli(&args)?;
        std::process::exit(exit_code);
    }

    let mut state = public_api::BackendState::new();
    let stdin = std::io::stdin();

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
        let request: protocol::request::Request = match io::json_codec::parse_request(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let resp = protocol::response::Response::Error {
                    message: e.to_string(),
                };
                protocol::message::write_response_stdout(&resp)?;
                continue;
            }
        };
        let response = state.handle(request);
        protocol::message::write_response_stdout(&response)?;
    }

    Ok(())
}
