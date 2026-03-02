// projects/products/unstable/protocol_builder/backend/src/protocol/message.rs
use serde::{Deserialize, Serialize};

/// A newline-delimited JSON framing envelope used on the IPC channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub payload: String,
}
