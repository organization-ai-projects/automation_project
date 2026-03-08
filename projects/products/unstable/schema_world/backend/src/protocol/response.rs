use crate::snapshots::snapshot::Snapshot;
use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum Response {
    Ok,
    Error { message: String },
    Snapshot { hash: String, snapshot: Snapshot },
    Diff { json: Json },
    Report { json: Json },
}
