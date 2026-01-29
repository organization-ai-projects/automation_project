// projects/products/core/engine/src/bootstrap/mod.rs

// Module declarations
mod bootstrap_error;
mod operations;
mod owner_claim;
mod setup_state;

// Re-exports from types
pub use bootstrap_error::BootstrapError;
pub use owner_claim::OwnerClaim;
pub use setup_state::SetupState;

// Re-exports from operations
pub use operations::{consume_claim, ensure_owner_claim, setup_complete, validate_claim};
