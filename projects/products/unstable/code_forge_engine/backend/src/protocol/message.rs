// projects/products/unstable/code_forge_engine/backend/src/protocol/message.rs
use crate::protocol::request::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub request: Request,
}
