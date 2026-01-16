use common_json::Json;
// projects/libraries/protocol/src/preview_response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewResponse {
    pub summary: String,
    pub payload: Option<Json>,
}
