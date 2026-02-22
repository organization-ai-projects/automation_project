// projects/libraries/core/contracts/protocol_accounts/src/accounts/create_account_request.rs
use serde::{Deserialize, Serialize};

use protocol::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: ProtocolId,
    pub password: String,
    pub role: String,
    pub permissions: Vec<String>,
}
