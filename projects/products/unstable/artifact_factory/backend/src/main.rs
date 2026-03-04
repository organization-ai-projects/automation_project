mod analyze;
mod bundle;
mod diagnostics;
mod input;
mod io;
mod protocol;
mod public_api;
mod render;

use crate::io::json_codec::JsonCodec;
use crate::protocol::Response;
use crate::protocol::message::{write_response_stdout, write_stderr_line};
use crate::public_api::BackendSession;
use std::io::BufRead as _;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("artifact-factory-backend starting");

    let stdin = std::io::stdin();
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

        if let Err(err) = write_response_stdout(id, response) {
            if let Err(stderr_err) = write_stderr_line(&format!("response write error: {err}")) {
                tracing::error!(error = %stderr_err, "failed to write stderr");
            }
            tracing::error!(error = %err, "failed to encode response");
        }
    }

    tracing::info!("artifact-factory-backend shutting down");
}
