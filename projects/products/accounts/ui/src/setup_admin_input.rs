// projects/products/accounts/ui/src/setup_admin_input.rs
use serde::Serialize;

/// Input data for setting up the first admin account
#[derive(Debug, Serialize)]
pub struct SetupAdminInput {
    pub user_id: String,
    pub password: String,
}
