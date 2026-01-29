// projects/products/core/engine/src/bootstrap/setup_state.rs
use std::path::PathBuf;

/// Tracks the state of the bootstrap/setup process
#[derive(Debug, Clone)]
pub struct SetupState {
    pub setup_mode: bool,
    pub claim_path: PathBuf,
    pub used_marker_path: PathBuf,
    pub expires_at: Option<i64>,
}
