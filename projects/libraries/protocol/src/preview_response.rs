// projects/libraries/protocol/src/preview_response.rs
use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewResponse {
    pub summary: String,
    pub payload: Option<Json>,
}
