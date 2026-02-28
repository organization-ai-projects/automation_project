// projects/products/unstable/code_forge_engine/backend/src/protocol/message.rs
use serde::{Deserialize, Serialize};
use crate::protocol::request::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub request: Request,
}
