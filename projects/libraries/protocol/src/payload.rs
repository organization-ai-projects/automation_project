use common_json::Json;
//projects/libraries/protocol/src/payload.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub payload_type: Option<String>,
    pub payload: Option<Json>,
}
