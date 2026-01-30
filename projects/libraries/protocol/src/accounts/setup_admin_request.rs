// projects/libraries/protocol/src/accounts/setup_admin_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAdminRequest {
    pub claim: String,
    pub user_id: String,
    pub password: String,
}
