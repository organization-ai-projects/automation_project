use serde::{Deserialize, Serialize};

use crate::metadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRequest {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub changes: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
