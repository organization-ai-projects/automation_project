use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerResponse {
    Ok,
    CompileReport {
        manifest_hash: String,
        report_json: String,
    },
    Error {
        message: String,
    },
}
