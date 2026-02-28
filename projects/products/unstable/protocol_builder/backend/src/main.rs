// projects/products/unstable/protocol_builder/backend/src/main.rs
mod diagnostics;
mod generate;
mod io;
mod output;
mod parse;
mod protocol;
mod public_api;
mod schema;

use anyhow::Result;
use parse::SchemaParser;
use protocol::{IpcRequest, IpcResponse};
use std::io::{BufRead, Write};

fn handle_request(
    request: IpcRequest,
    state: &mut BackendState,
) -> IpcResponse {
    match request {
        IpcRequest::LoadSchema { path } => match SchemaParser::parse_file(&path) {
            Ok(s) => {
                tracing::info!(path = %path, "schema loaded");
                state.schema = Some(s);
                state.manifest = None;
                IpcResponse::Ok
            }
            Err(e) => IpcResponse::Error { message: e.to_string() },
        },
        IpcRequest::ValidateSchema => match &state.schema {
            None => IpcResponse::Error { message: "no schema loaded".to_string() },
            Some(schema) => match public_api::validate_schema(schema) {
                Ok(()) => IpcResponse::Ok,
                Err(msg) => IpcResponse::Error { message: msg },
            },
        },
        IpcRequest::GenerateDryRun => match &state.schema {
            None => IpcResponse::Error { message: "no schema loaded".to_string() },
            Some(schema) => {
                let manifest = public_api::build_manifest(schema);
                let report = public_api::build_report(&manifest);
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                state.manifest = Some(manifest);
                IpcResponse::GenerateReport { manifest_hash, report_json }
            }
        },
        IpcRequest::GenerateWrite { out_dir } => match &state.schema {
            None => IpcResponse::Error { message: "no schema loaded".to_string() },
            Some(schema) => {
                let manifest = public_api::build_manifest(schema);
                let report = public_api::build_report(&manifest);
                for (name, content) in &manifest.artifacts {
                    if let Err(e) = io::write_atomic(&out_dir, name, content) {
                        return IpcResponse::Error { message: e.to_string() };
                    }
                }
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                state.manifest = Some(manifest);
                IpcResponse::GenerateReport { manifest_hash, report_json }
            }
        },
        IpcRequest::GetReport => match &state.manifest {
            None => IpcResponse::Error { message: "no artifacts generated yet".to_string() },
            Some(manifest) => {
                let report = public_api::build_report(manifest);
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                IpcResponse::GenerateReport { manifest_hash, report_json }
            }
        },
        IpcRequest::Shutdown => {
            tracing::info!("shutdown requested");
            std::process::exit(0);
        }
    }
}

struct BackendState {
    schema: Option<crate::schema::ProtocolSchema>,
    manifest: Option<crate::output::ArtifactManifest>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("protocol_builder backend starting");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut state = BackendState { schema: None, manifest: None };

    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let response = match io::decode::<IpcRequest>(line) {
            Ok(req) => handle_request(req, &mut state),
            Err(e) => IpcResponse::Error { message: format!("parse error: {}", e) },
        };
        let encoded = io::encode(&response)?;
        writeln!(stdout, "{}", encoded)?;
        stdout.flush()?;
    }

    Ok(())
}
