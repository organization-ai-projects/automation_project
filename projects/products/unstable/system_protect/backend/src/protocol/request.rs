use serde::{Deserialize, Serialize};

use super::request_payload::RequestPayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub payload: RequestPayload,
}

impl Request {
    #[allow(dead_code)]
    pub fn new(payload: RequestPayload) -> Self {
        Self { payload }
    }
}
