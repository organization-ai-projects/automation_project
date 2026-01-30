// projects/libraries/protocol/src/accounts/update_account_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub role: Option<String>,
    pub permissions: Option<Vec<String>>,
}
