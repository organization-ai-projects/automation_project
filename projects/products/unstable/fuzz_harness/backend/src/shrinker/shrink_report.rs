use crate::model::FuzzInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ShrinkReport {
    pub(crate) target_name: String,
    pub(crate) original_size: usize,
    pub(crate) shrunk_input: FuzzInput,
    pub(crate) shrink_steps: u64,
    pub(crate) failure_message: String,
}
