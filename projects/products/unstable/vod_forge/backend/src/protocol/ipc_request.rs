// projects/products/unstable/vod_forge/backend/src/protocol/ipc_request.rs
use serde::{Deserialize, Serialize};

use crate::protocol::RequestPayload;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: u64,
    pub payload: RequestPayload,
}
