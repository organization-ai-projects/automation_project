use crate::migrate::migration::Migration;
use crate::schemas::schema::Schema;
use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum Request {
    LoadSchema { schema: Schema },
    ValidateSchema,
    Insert { record: Json },
    Update { id: u64, record: Json },
    Delete { id: u64 },
    Snapshot,
    Diff { from: Json, to: Json },
    Migrate { id: u64, migration: Migration },
    Report,
    Shutdown,
}
