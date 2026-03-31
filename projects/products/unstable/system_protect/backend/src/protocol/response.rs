use serde::{Deserialize, Serialize};

use super::response_payload::ResponsePayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub payload: ResponsePayload,
}

impl Response {
    pub fn new(payload: ResponsePayload) -> Self {
        Self { payload }
    }
}
