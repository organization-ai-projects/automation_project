// projects/products/stable/accounts/ui/src/setup_admin_input.rs
use protocol::ProtocolId;
use serde::Serialize;

//replace user_id issue #67
/// Input data for setting up the first admin account
#[derive(Debug, Serialize)]
pub struct SetupAdminInput {
    pub user_id: ProtocolId,
    pub password: String,
}
