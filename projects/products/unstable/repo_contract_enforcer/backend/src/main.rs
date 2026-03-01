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

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] != "serve" {
        eprintln!("usage: repo_contract_enforcer_backend serve");
        std::process::exit(2);
    }

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

        let request: protocol::message::Request = match common_json::from_json_str(line.trim()) {
            Ok(req) => req,
            Err(err) => {
                let wrapped = diagnostics::error::Error::Internal(err.to_string());
                let response: protocol::message::Response = protocol::response::Response {
                    id: None,
                    payload: protocol::message::ResponsePayload::Error {
                        code: "INVALID_REQUEST_JSON".to_string(),
                        message: "failed to parse request".to_string(),
                        details: Some(wrapped.to_string()),
                    },
                };
                io::json_codec::JsonCodec::write_response(&stdout, &response)?;
                continue;
            }
        };

        let response = state.handle(request);
        io::json_codec::JsonCodec::write_response(&stdout, &response)?;

        if state.should_shutdown() {
            break;
        }
    }

    Ok(())
}
