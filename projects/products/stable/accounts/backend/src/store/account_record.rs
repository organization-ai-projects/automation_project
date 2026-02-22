// projects/products/stable/accounts/backend/src/store/account_record.rs
use protocol::ProtocolId;
use protocol_accounts::AccountStatus;
use security::{Permission, Role};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountRecord {
    pub user_id: ProtocolId,
    pub password_hash: String,
    pub role: Role,
    pub extra_permissions: Vec<Permission>,
    pub status: AccountStatus,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub last_login_ms: Option<u64>,
}
