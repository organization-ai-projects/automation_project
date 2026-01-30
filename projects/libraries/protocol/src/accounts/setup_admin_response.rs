// projects/libraries/protocol/src/accounts/setup_admin_response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupAdminResponse {
    pub ok: bool,
}
