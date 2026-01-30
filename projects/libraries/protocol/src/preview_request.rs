// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;

//replace request_id issue #67
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewRequest {
    #[serde(rename = "request_id")]
    pub request_id: String,
    pub details: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
