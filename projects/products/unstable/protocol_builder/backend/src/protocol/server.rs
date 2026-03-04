// projects/products/unstable/protocol_builder/backend/src/protocol/server.rs
use anyhow::Result;
use std::io::{BufRead, Write};

use crate::diagnostics::backend_error::BackendError;
use crate::io;
use crate::parse::SchemaParser;
use crate::public_api;

use super::message::Message;
use super::server_state::ServerState;
use super::{Payload, Request, Response};

fn handle_request(request: Request, state: &mut ServerState) -> Response {
    match request {
        Request::LoadSchema { path } => match SchemaParser::parse_file(&path) {
            Ok(s) => {
                tracing::info!(path = %path, "schema loaded");
                state.schema = Some(s);
                state.manifest = None;
                Response::Ok
            }
            Err(e) => Response::Error {
                message: e.to_string(),
            },
        },
        Request::ValidateSchema => match &state.schema {
            None => Response::Error {
                message: BackendError::NoSchemaLoaded.to_string(),
            },
            Some(schema) => match public_api::validate_schema(schema) {
                Ok(()) => Response::Ok,
                Err(msg) => Response::Error { message: msg },
            },
        },
        Request::GenerateDryRun => match &state.schema {
            None => Response::Error {
                message: BackendError::NoSchemaLoaded.to_string(),
            },
            Some(schema) => {
                let manifest = public_api::build_manifest(schema);
                let report = public_api::build_report(&manifest);
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                state.manifest = Some(manifest);
                Response::GenerateReport {
                    manifest_hash,
                    report_json,
                }
            }
        },
        Request::GenerateWrite { out_dir } => match &state.schema {
            None => Response::Error {
                message: BackendError::NoSchemaLoaded.to_string(),
            },
            Some(schema) => {
                let manifest = public_api::build_manifest(schema);
                let report = public_api::build_report(&manifest);
                for (name, content) in &manifest.artifacts {
                    if let Err(e) = io::write_atomic(&out_dir, name, content) {
                        return Response::Error {
                            message: e.to_string(),
                        };
                    }
                }
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                state.manifest = Some(manifest);
                Response::GenerateReport {
                    manifest_hash,
                    report_json,
                }
            }
        },
        Request::GetReport => match &state.manifest {
            None => Response::Error {
                message: BackendError::SchemaInvalid {
                    reason: "no artifacts generated yet".to_string(),
                }
                .to_string(),
            },
            Some(manifest) => {
                let report = public_api::build_report(manifest);
                let report_json = common_json::to_string(&report).unwrap_or_default();
                let manifest_hash = report.manifest_hash.clone();
                Response::GenerateReport {
                    manifest_hash,
                    report_json,
                }
            }
        },
        Request::Shutdown => Response::Ok,
    }
}

fn decode_request_message(line: &str) -> Result<Message, BackendError> {
    let message: Message =
        io::decode(line).map_err(|e| BackendError::Json(format!("invalid IPC JSON: {e}")))?;
    Ok(message)
}

pub fn run() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut state = ServerState {
        schema: None,
        manifest: None,
    };

    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut should_shutdown = false;
        let response_message = match decode_request_message(line) {
            Ok(request_message) => {
                let response_payload = match request_message.payload {
                    Payload::Request(req) => {
                        if matches!(req, Request::Shutdown) {
                            should_shutdown = true;
                        }
                        handle_request(req, &mut state)
                    }
                    Payload::Response(_) => Response::Error {
                        message: "unexpected response payload from UI".to_string(),
                    },
                };
                Message {
                    id: request_message.id,
                    payload: Payload::Response(response_payload),
                }
            }
            Err(err) => Message {
                id: 0,
                payload: Payload::Response(Response::Error {
                    message: err.to_string(),
                }),
            },
        };
        let encoded = io::encode(&response_message)?;
        writeln!(stdout, "{encoded}")?;
        stdout.flush()?;
        if should_shutdown {
            tracing::info!("shutdown requested");
            break;
        }
    }

    Ok(())
}
