use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupStatusResponse {
    pub setup_mode: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAdminRequest {
    pub claim: String,
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAdminResponse {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String,
    pub role: Option<String>,
    pub duration_ms: Option<u64>,
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSummary {
    pub user_id: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub status: String,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub last_login_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsListResponse {
    pub users: Vec<AccountSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: String,
    pub password: String,
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub role: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}
