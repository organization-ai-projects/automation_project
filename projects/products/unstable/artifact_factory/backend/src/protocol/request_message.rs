use crate::protocol::request::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
    pub id: u64,
    pub request: Request,
}
