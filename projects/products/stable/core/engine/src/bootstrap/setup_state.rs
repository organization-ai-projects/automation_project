// projects/products/stable/core/engine/src/bootstrap/setup_state.rs
use std::path::PathBuf;

//use common_time to expires_at
/// Tracks the state of the bootstrap/setup process
#[derive(Debug, Clone)]
pub(crate) struct SetupState {
    pub(crate) setup_mode: bool,
    pub(crate) claim_path: PathBuf,
    pub(crate) used_marker_path: PathBuf,
    pub(crate) expires_at: Option<i64>,
}
