// projects/libraries/protocol/src/run_request.rs
use serde::{Deserialize, Serialize};

use crate::protocol_id::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunRequest {
    #[serde(rename = "request_id")]
    pub request_id: ProtocolId,
    /// Optional repository path. If not provided, falls back to environment or current directory.
    pub repo_path: Option<String>,
}
