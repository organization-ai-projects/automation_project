// projects/products/unstable/vod_forge/backend/src/protocol/ipc_response.rs
use serde::{Deserialize, Serialize};

use crate::protocol::ResponsePayload;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub payload: ResponsePayload,
}
