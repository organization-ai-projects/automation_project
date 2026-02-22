// projects/libraries/core/contracts/protocol_accounts/src/accounts/setup_admin_request.rs
use serde::{Deserialize, Serialize};

use protocol::ProtocolId;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAdminRequest {
    pub claim: String,
    pub user_id: ProtocolId,
    pub password: String,
}
