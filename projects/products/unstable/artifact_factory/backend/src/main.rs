mod analyze;
mod bundle;
mod diagnostics;
mod input;
mod io;
mod protocol;
mod public_api;
mod render;

use crate::io::json_codec::JsonCodec;
use crate::protocol::{RequestMessage, Response, ResponseMessage};
use crate::public_api::BackendSession;
use std::io::BufRead as _;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("artifact-factory-backend starting");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut session = BackendSession::default();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) if l.trim().is_empty() => continue,
            Ok(l) => l,
            Err(e) => {
                tracing::error!(error = %e, "stdin read error");
                break;
            }
        };

        let msg = match JsonCodec::decode_request(&line) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!(error = %e, "failed to decode request");
                continue;
            }
        };

        let id = msg.id;
        let response = match session.handle(msg.request) {
            Ok(r) => r,
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        };

        let resp_msg = ResponseMessage { id, response };
        match JsonCodec::encode_response(&resp_msg) {
            Ok(encoded) => {
                use std::io::Write as _;
                let mut out = stdout.lock();
                let _ = writeln!(out, "{}", encoded);
            }
            Err(e) => tracing::error!(error = %e, "failed to encode response"),
        }
    }

    tracing::info!("artifact-factory-backend shutting down");
}
