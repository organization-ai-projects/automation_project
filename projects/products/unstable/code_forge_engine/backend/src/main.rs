// projects/products/unstable/code_forge_engine/backend/src/main.rs
mod contract;
mod diagnostics;
mod generate;
mod io;
mod output;
mod protocol;
mod public_api;
mod render;
mod validate;

use anyhow::Result;
use diagnostics::error::ForgeError;
use io::json_codec::JsonCodec;
use protocol::message::Message;
use protocol::request::Request;
use protocol::response::Response;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) != Some("serve") {
        eprintln!("Usage: code_forge_engine_backend serve");
        std::process::exit(2);
    }
    serve_loop()
}

fn serve_loop() -> Result<()> {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let codec = JsonCodec::new();
    let mut state = ServerState::new();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let msg: Message = match codec.decode(&line) {
            Ok(m) => m,
            Err(e) => {
                let resp = Response::error(2, "decode_error", &e.to_string());
                codec.write_line(&stdout, &resp)?;
                continue;
            }
        };
        let resp = state.handle(msg.request);
        codec.write_line(&stdout, &resp)?;
        if matches!(resp, Response::Ok) && state.shutdown_requested {
            break;
        }
    }
    Ok(())
}

struct ServerState {
    contract_path: Option<String>,
    shutdown_requested: bool,
}

impl ServerState {
    fn new() -> Self {
        Self { contract_path: None, shutdown_requested: false }
    }

    fn handle(&mut self, req: Request) -> Response {
        match req {
            Request::LoadContract { path } => {
                self.contract_path = Some(path);
                Response::Ok
            }
            Request::ValidateContract => Response::Ok,
            Request::PreviewLayout => Response::Preview { files: vec![] },
            Request::Generate { out_dir: _, mode: _ } => Response::Ok,
            Request::GetManifest => Response::Manifest {
                manifest_json: "{}".to_string(),
                manifest_hash: "".to_string(),
            },
            Request::Shutdown => {
                self.shutdown_requested = true;
                Response::Ok
            }
        }
    }
}
