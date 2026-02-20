// projects/libraries/protocol/src/accounts/create_account_request.rs
use serde::{Deserialize, Serialize};

use crate::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: ProtocolId,
    pub password: String,
    pub role: String,
    pub permissions: Vec<String>,
}
