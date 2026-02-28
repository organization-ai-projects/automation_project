// projects/products/unstable/code_forge_engine/backend/src/protocol/response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok,
    Error { code: u32, message: String, details: String },
    Preview { files: Vec<String> },
    Manifest { manifest_json: String, manifest_hash: String },
}

impl Response {
    pub fn error(code: u32, message: &str, details: &str) -> Self {
        Self::Error {
            code,
            message: message.to_string(),
            details: details.to_string(),
        }
    }
}
