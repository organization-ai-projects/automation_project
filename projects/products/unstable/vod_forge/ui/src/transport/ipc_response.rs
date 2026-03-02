use crate::transport::response_payload::ResponsePayload;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IpcResponse {
    pub id: u64,
    pub payload: ResponsePayload,
}
