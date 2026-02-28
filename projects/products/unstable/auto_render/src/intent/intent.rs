use serde::{Deserialize, Serialize};
use super::{IntentId, IntentVersion, IntentPayload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_id: IntentId,
    pub intent_version: IntentVersion,
    pub created_at: String,
    pub author: Option<String>,
    pub payload: IntentPayload,
}
