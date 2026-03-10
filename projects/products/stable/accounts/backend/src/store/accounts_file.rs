//! projects/products/stable/accounts/backend/src/store/accounts_file.rs
use crate::store::AccountRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountsFile {
    pub schema_version: u32,
    pub users: Vec<AccountRecord>,
}
