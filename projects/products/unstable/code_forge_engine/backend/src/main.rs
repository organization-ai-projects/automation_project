mod backend_session;
mod contract;
mod diagnostics;
mod generate;
mod io;
mod output;
mod protocol;
mod public_api;
mod validate;

use crate::public_api::BackendSession;
use anyhow::Result;
use io::json_codec::JsonCodec;
use protocol::message::{Message, write_response_stdout};
use protocol::response::Response;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) != Some("serve") {
        std::process::exit(2);
    }
    serve_loop()
}

fn serve_loop() -> Result<()> {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let codec = JsonCodec::new();
    let mut session = BackendSession::default();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let msg: Message = match codec.decode(&line) {
            Ok(message) => message,
            Err(error) => {
                let response = Response::error(2, "decode_error", &error.to_string());
                write_response_stdout(&codec, &response)?;
                continue;
            }
        };

        let response = session.handle(msg.request);
        write_response_stdout(&codec, &response)?;

        if matches!(response, Response::Ok) && session.should_shutdown() {
            break;
        }
    }

    Ok(())
}
