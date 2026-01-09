use serde::{Deserialize, Serialize};

use crate::metadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewRequest {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub details: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
