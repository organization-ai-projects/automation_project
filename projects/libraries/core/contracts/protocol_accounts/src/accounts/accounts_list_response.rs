// projects/libraries/core/contracts/protocol_accounts/src/accounts/accounts_list_response.rs
use serde::{Deserialize, Serialize};

use crate::AccountSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsListResponse {
    pub users: Vec<AccountSummary>,
}
