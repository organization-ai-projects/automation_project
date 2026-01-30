// projects/libraries/protocol/src/accounts/update_status_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}
