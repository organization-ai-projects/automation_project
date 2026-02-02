// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;
use crate::protocol_id::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRequest {
    #[serde(rename = "request_id")]
    pub request_id: ProtocolId,
    pub changes: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
