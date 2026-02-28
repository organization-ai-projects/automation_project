mod analytics;
mod catalog;
mod diagnostics;
mod io;
mod packaging;
mod playback;
mod protocol;
mod public_api;
mod recommend;

use io::JsonCodec;
use protocol::{IpcRequest, IpcResponse, ResponsePayload};
use public_api::PublicApi;
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] != "serve" {
        eprintln!("Usage: vod_forge_backend serve");
        std::process::exit(2);
    }

    eprintln!("vod_forge_backend: starting IPC loop");
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut api = PublicApi::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let response: IpcResponse = match JsonCodec::decode::<IpcRequest>(line) {
            Err(e) => IpcResponse {
                id: 0,
                payload: ResponsePayload::Error { message: format!("decode error: {}", e) },
            },
            Ok(req) => api.handle_request(req),
        };
        match JsonCodec::encode(&response) {
            Err(e) => eprintln!("encode error: {}", e),
            Ok(json) => {
                let _ = writeln!(out, "{}", json);
                let _ = out.flush();
            }
        }
    }
    eprintln!("vod_forge_backend: IPC loop ended");
}
