use crate::canon::canonical_decoder::CanonicalDecoder;
use crate::diagnostics::error::Error;
use crate::io::json_codec;
use crate::migrate::migration_engine::MigrationEngine;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::reports::report::Report;
use crate::schemas::schema::Schema;
use crate::snapshots::snapshot::Snapshot;
use crate::snapshots::snapshot_hash::SnapshotHash;
use crate::storage::record_store::RecordStore;
use crate::validate::data_validator::DataValidator;
use crate::validate::schema_validator::SchemaValidator;
use crate::{canon::canonical_encoder::CanonicalEncoder, diff::diff_engine::DiffEngine};
use common_json::Json;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub id: u64,
    pub payload: T,
}

pub fn run_server_loop() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut schema = Schema::default();
    let mut store = RecordStore::default();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let req_msg: Message<Request> = match json_codec::from_json(&line) {
            Ok(msg) => msg,
            Err(err) => {
                let response = Message {
                    id: 0,
                    payload: Response::Error {
                        message: Error::Json(format!("invalid request: {err}")).to_string(),
                    },
                };
                let response_json =
                    json_codec::to_json(&response).unwrap_or_else(|_| "{}".to_string());
                writeln!(stdout, "{response_json}")?;
                stdout.flush()?;
                continue;
            }
        };

        let is_shutdown = matches!(req_msg.payload, Request::Shutdown);
        let response_payload = handle_request(req_msg.payload, &mut schema, &mut store);
        let response = Message {
            id: req_msg.id,
            payload: response_payload,
        };

        let response_json = json_codec::to_json(&response).unwrap_or_else(|_| "{}".to_string());
        writeln!(stdout, "{response_json}")?;
        stdout.flush()?;

        if matches!(response.payload, Response::Ok) && is_shutdown {
            break;
        }
    }

    Ok(())
}

fn handle_request(request: Request, schema: &mut Schema, store: &mut RecordStore) -> Response {
    match request {
        Request::LoadSchema { schema: next } => {
            *schema = next;
            Response::Ok
        }
        Request::ValidateSchema => validate_schema(schema)
            .map(|_| Response::Ok)
            .unwrap_or_else(validation_response),
        Request::Insert { record } => {
            if let Err(message) = validate_record(schema, &record) {
                return validation_response(message);
            }
            store.insert(record);
            Response::Ok
        }
        Request::Update { id, record } => {
            if let Err(message) = validate_record(schema, &record) {
                return validation_response(message);
            }
            if store.update(id, record) {
                Response::Ok
            } else {
                Response::Error {
                    message: format!("record {id} not found"),
                }
            }
        }
        Request::Delete { id } => {
            if store.delete(id) {
                Response::Ok
            } else {
                Response::Error {
                    message: format!("record {id} not found"),
                }
            }
        }
        Request::Snapshot => match snapshot_hash(store, schema.version.value) {
            Ok(hash) => Response::Snapshot {
                hash,
                snapshot: Snapshot::from_store(schema.version.value, store),
            },
            Err(message) => Response::Error { message },
        },
        Request::Diff { from, to } => {
            let from_snapshot: Snapshot = match common_json::from_value(from) {
                Ok(snapshot) => snapshot,
                Err(err) => {
                    return Response::Error {
                        message: format!("invalid from snapshot: {err}"),
                    };
                }
            };
            let to_snapshot: Snapshot = match common_json::from_value(to) {
                Ok(snapshot) => snapshot,
                Err(err) => {
                    return Response::Error {
                        message: format!("invalid to snapshot: {err}"),
                    };
                }
            };
            Response::Diff {
                json: diff(&from_snapshot, &to_snapshot),
            }
        }
        Request::Migrate { id, migration } => {
            let current = match store.get(id) {
                Some(record) => record.data.clone(),
                None => {
                    return Response::Error {
                        message: format!("record {id} not found"),
                    };
                }
            };
            match MigrationEngine::apply_forward(&current, &migration) {
                Ok(next) => {
                    let round_trip = match MigrationEngine::apply_reverse(&next, &migration) {
                        Ok(previous) => previous,
                        Err(message) => return migration_response(message),
                    };
                    if round_trip != current {
                        return migration_response("migration round-trip check failed".to_string());
                    }
                    if let Err(message) = validate_record(schema, &next) {
                        return validation_response(message);
                    }
                    store.update(id, next);
                    Response::Ok
                }
                Err(message) => migration_response(message),
            }
        }
        Request::Report => match build_report(store, schema.version.value) {
            Ok(report) => Response::Report {
                json: common_json::to_value(&report).unwrap_or(Json::Null),
            },
            Err(message) => Response::Error { message },
        },
        Request::Shutdown => Response::Ok,
    }
}

fn validate_schema(schema: &Schema) -> Result<(), String> {
    SchemaValidator::validate(schema)
}

fn validate_record(schema: &Schema, record: &Json) -> Result<(), String> {
    DataValidator::validate_record(schema, record)
}

fn snapshot_hash(store: &RecordStore, schema_version: u32) -> Result<String, String> {
    let snapshot = Snapshot::from_store(schema_version, store);
    let bytes = CanonicalEncoder::encode_snapshot(&snapshot)?;
    CanonicalDecoder::decode_value(&bytes)?;
    Ok(SnapshotHash::from_bytes(&bytes).to_hex())
}

fn diff(from: &Snapshot, to: &Snapshot) -> Json {
    DiffEngine::diff(from, to)
}

fn build_report(store: &RecordStore, schema_version: u32) -> Result<Report, String> {
    let snapshot = Snapshot::from_store(schema_version, store);
    let bytes = CanonicalEncoder::encode_snapshot(&snapshot)?;
    let hash = SnapshotHash::from_bytes(&bytes).to_hex();
    Ok(Report {
        schema_version,
        record_count: snapshot.records.len(),
        snapshot_hash: hash,
        canonical_bytes_len: bytes.len() as u64,
    })
}

fn validation_response(message: String) -> Response {
    Response::Error {
        message: Error::Validation(message).to_string(),
    }
}

fn migration_response(message: String) -> Response {
    Response::Error {
        message: Error::Migration(message).to_string(),
    }
}
