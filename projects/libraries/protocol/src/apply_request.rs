// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;
use crate::protocol_id::ProtocolId;
use common::custom_uuid::Id128;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRequest {
    #[serde(rename = "request_id")]
    pub request_id: Id128,
    pub changes: String,
    pub policy_overrides: Option<metadata::Metadata>,
}

impl ApplyRequest {
    /// Create an ApplyRequest using a ProtocolId for future-compatibility.
    pub fn with_protocol_id(
        request_id: ProtocolId,
        changes: String,
        policy_overrides: Option<metadata::Metadata>,
    ) -> Self {
        Self {
            request_id: request_id.as_inner(),
            changes,
            policy_overrides,
        }
    }

    /// Convert the stored request_id into a ProtocolId.
    pub fn request_id_protocol_id(&self) -> ProtocolId {
        ProtocolId::new(self.request_id)
    }
}
