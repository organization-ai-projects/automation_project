// projects/products/accounts/backend/src/store/account_record.rs
use protocol::accounts::AccountStatus;
use security::{Permission, Role};

//replace user_id issue #67
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountRecord {
    pub user_id: String,
    pub password_hash: String,
    pub role: Role,
    pub extra_permissions: Vec<Permission>,
    pub status: AccountStatus,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub last_login_ms: Option<u64>,
}
