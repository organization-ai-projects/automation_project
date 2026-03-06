mod backend_session;
mod constraints;
mod determinism;
mod diagnostics;
mod dsl;
mod events;
mod io;
mod model;
mod protocol;
mod public_api;
mod replay;
mod report;
mod snapshots;
mod solve;
mod transitions;

use std::{env, process};

use crate::io::json_codec::JsonCodec;
use crate::protocol::response::Response;
use crate::protocol::response_payload::ResponsePayload;
use crate::public_api::BackendSession;
use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1] != "serve" {
        process::exit(2);
    }

    let mut state = BackendSession::default();
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
