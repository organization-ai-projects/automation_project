//projects/libraries/protocol/src/payload.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub payload_type: Option<String>,
    pub payload: Option<Value>,
}
