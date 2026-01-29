// projects/products/core/engine/src/bootstrap/owner_claim.rs
use serde::{Deserialize, Serialize};

/// Owner bootstrap claim with secret and expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerClaim {
    pub secret: String,
    pub created_at: i64,
    pub expires_at: i64,
}
