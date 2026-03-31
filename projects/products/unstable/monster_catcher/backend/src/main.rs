mod backend_session;
mod capture;
mod combat;
mod data;
mod diagnostics;
mod encounter;
mod events;
mod io;
mod model;
mod progression;
mod protocol;
mod public_api;
mod replay;
mod report;
mod rng;
mod scenario;
mod snapshot;

use crate::io::json_codec::JsonCodec;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;
use crate::public_api::BackendSession;
use anyhow::Result;
use std::{env, process};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let has_serve = args.iter().any(|a| a == "serve");
    if !has_serve {
        eprintln!("Usage: monster_catcher_backend serve [--scenario <file>]");
        process::exit(2);
    }

    let scenario_path = args
        .iter()
        .position(|a| a == "--scenario")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let mut state = BackendSession::new(scenario_path);
    let stdin = std::io::stdin();

    loop {
        let mut line = String::new();
        let n = {
            use std::io::BufRead;
            stdin.lock().read_line(&mut line)?
        };
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let request = match JsonCodec::parse_request(trimmed) {
            Ok(req) => req,
            Err(err) => {
                let response = Response {
                    id: None,
                    payload: ResponsePayload::Error {
                        message: err.to_string(),
                    },
                };
                protocol::message::write_response_stdout(&response)?;
                continue;
            }
        };

        let response = state.handle(request);
        protocol::message::write_response_stdout(&response)?;

        if state.should_shutdown() {
            break;
        }
    }

    Ok(())
}
