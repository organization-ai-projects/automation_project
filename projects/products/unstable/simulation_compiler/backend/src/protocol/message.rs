// projects/products/unstable/simulation_compiler/backend/src/protocol/message.rs
use serde::{Deserialize, Serialize};

use super::request::CompilerRequest;
use super::response::CompilerResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub id: u64,
    pub payload: IpcPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "direction")]
pub enum IpcPayload {
    Request(CompilerRequest),
    Response(CompilerResponse),
}
