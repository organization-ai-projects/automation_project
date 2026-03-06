// projects/products/unstable/protocol_builder/backend/src/protocol/response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Response {
    Ok,
    Error {
        message: String,
    },
    GenerateReport {
        manifest_hash: String,
        report_json: String,
    },
}
