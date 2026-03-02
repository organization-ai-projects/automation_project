// projects/products/unstable/digital_pet/backend/src/protocol/message.rs
use crate::protocol::request::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<u64>,
    pub request: Request,
}
