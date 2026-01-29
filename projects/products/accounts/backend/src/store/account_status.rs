// projects/products/accounts/backend/src/store/status.rs
use crate::store::account_store_error::AccountStoreError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Suspended,
    Disabled,
}

impl AccountStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountStatus::Active => "active",
            AccountStatus::Suspended => "suspended",
            AccountStatus::Disabled => "disabled",
        }
    }
}

impl FromStr for AccountStatus {
    type Err = AccountStoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "active" => Ok(AccountStatus::Active),
            "suspended" => Ok(AccountStatus::Suspended),
            "disabled" => Ok(AccountStatus::Disabled),
            _ => Err(AccountStoreError::InvalidStatus),
        }
    }
}
