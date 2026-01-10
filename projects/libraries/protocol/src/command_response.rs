//projects/libraries/protocol/src/command_response.rs
use crate::{ProtocolError, ResponseStatus, metadata::Metadata, payload::Payload}; // Import Metadata and Payload
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub metadata: Metadata,
    pub status: ResponseStatus,
    pub message: Option<String>,
    pub payload: Option<Payload>,
    pub error: Option<ProtocolError>,
}
