//projects/products/core/central_ui/src/owner_claim.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct OwnerClaim {
    pub(crate) secret: String,
}
