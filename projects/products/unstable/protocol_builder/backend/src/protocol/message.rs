// projects/products/unstable/protocol_builder/backend/src/protocol/message.rs
use serde::{Deserialize, Serialize};

use super::payload::Payload;

/// A newline-delimited JSON framing envelope used on the IPC channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub payload: Payload,
}
