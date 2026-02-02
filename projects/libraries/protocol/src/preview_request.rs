// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;
use crate::protocol_id::ProtocolId;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewRequest {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub details: String,
    pub policy_overrides: Option<metadata::Metadata>,
}

impl PreviewRequest {
    /// Create a PreviewRequest using a ProtocolId for future-compatibility.
    pub fn with_protocol_id(
        request_id: ProtocolId,
        details: String,
        policy_overrides: Option<metadata::Metadata>,
    ) -> Self {
        Self {
            request_id: request_id.to_hex(),
            details,
            policy_overrides,
        }
    }

    /// Parse the stored request_id into a ProtocolId.
    pub fn request_id_protocol_id(&self) -> Result<ProtocolId, <ProtocolId as FromStr>::Err> {
        ProtocolId::from_str(&self.request_id)
    }
}
