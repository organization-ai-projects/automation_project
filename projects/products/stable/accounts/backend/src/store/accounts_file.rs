// projects/products/stable/accounts/backend/src/store/accounts_file.rs
use crate::store::account_record::AccountRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountsFile {
    pub schema_version: u32,
    pub users: Vec<AccountRecord>,
}
