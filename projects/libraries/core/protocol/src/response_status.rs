// projects/libraries/protocol/src/response_status.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseStatus {
    pub code: u16,
    pub description: String,
}
