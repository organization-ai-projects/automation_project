// projects/products/unstable/protocol_builder/ui/src/transport/message.rs
use serde::{Deserialize, Serialize};

use super::payload::Payload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub payload: Payload,
}
