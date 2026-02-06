// projects/products/unstable/auto_manager_ai/src/domain/run_output.rs

use serde::{Deserialize, Serialize};

/// Output information from the run
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunOutput {
    pub actions_proposed: usize,
    pub actions_allowed: usize,
    pub actions_denied: usize,
    pub actions_needs_input: usize,
}
