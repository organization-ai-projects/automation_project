// projects/libraries/protocol/src/accounts/reset_password_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}
