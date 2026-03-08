use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum Request {
    LoadSchema { schema: Json },
    Insert { record: Json },
    Snapshot,
    Report,
    Shutdown,
}
