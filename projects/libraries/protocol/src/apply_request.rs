// projects/libraries/protocol/src/preview_request.rs
use serde::{Deserialize, Serialize};

use crate::metadata;
use common::custom_uuid::Id128;

//replace request_id issue #67
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyRequest {
    #[serde(rename = "request_id")]
    pub request_id: Id128,
    pub changes: String,
    pub policy_overrides: Option<metadata::Metadata>,
}
