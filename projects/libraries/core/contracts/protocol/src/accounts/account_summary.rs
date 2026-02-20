// projects/libraries/protocol/src/accounts/account_summary.rs
use serde::{Deserialize, Serialize};

use crate::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSummary {
    pub user_id: ProtocolId,
    pub role: String,
    pub permissions: Vec<String>,
    pub status: String,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub last_login_ms: Option<u64>,
}
