// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;
use crate::protocol_id::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewRequest {
    #[serde(rename = "request_id")]
    pub request_id: ProtocolId,
    pub details: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
