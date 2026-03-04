use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok,
    Error {
        code: u32,
        message: String,
        details: String,
    },
    Preview {
        files: Vec<String>,
    },
    Manifest {
        manifest_json: String,
        manifest_hash: String,
    },
}
