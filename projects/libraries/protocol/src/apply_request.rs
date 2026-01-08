use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRequest {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub changes: String,
    pub policy_overrides: Option<crate::metadata::Metadata>,
}
