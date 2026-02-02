// projects/libraries/protocol/src/accounts/login_request.rs
use serde::{Deserialize, Serialize};

use crate::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user_id: ProtocolId,
    pub password: String,
    pub role: Option<String>,
    pub duration_ms: Option<u64>,
    pub session_id: Option<ProtocolId>,
}
