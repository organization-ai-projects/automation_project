// projects/libraries/protocol/src/protocol_error.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtocolError {
    pub code: i32,
    pub message: String,
}
