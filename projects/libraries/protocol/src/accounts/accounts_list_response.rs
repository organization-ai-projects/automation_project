// projects/libraries/protocol/src/accounts/accounts_list_response.rs
use serde::{Deserialize, Serialize};

use crate::accounts::AccountSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsListResponse {
    pub users: Vec<AccountSummary>,
}
