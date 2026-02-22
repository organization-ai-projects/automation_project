// projects/libraries/core/contracts/protocol_accounts/src/accounts/login_response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}
