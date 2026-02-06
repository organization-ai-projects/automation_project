// projects/products/stable/core/engine/src/bootstrap/owner_claim.rs
use serde::{Deserialize, Serialize};

// use common_time for created_at and expires_at
/// Owner bootstrap claim with secret and expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OwnerClaim {
    pub(crate) secret: String,
    pub(crate) created_at: i64,
    pub(crate) expires_at: i64,
}
