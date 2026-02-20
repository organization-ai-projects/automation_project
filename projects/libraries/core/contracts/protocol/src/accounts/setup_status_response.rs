// projects/libraries/protocol/src/accounts/setup_status_response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupStatusResponse {
    pub setup_mode: bool,
}
