use crate::protocol::response_payload::ResponsePayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: ResponsePayload,
}
