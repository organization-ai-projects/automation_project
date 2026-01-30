// projects/libraries/protocol/src/accounts/account_status.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

impl std::str::FromStr for AccountStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "active" => Ok(AccountStatus::Active),
            "suspended" => Ok(AccountStatus::Suspended),
            "disabled" => Ok(AccountStatus::Disabled),
            _ => Err("Invalid status".to_string()),
        }
    }
}

// Display implementation for AccountStatus
impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
