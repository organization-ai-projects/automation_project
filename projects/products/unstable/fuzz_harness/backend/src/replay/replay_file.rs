use crate::model::FuzzInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ReplayFile {
    pub(crate) target_name: String,
    pub(crate) seed: u64,
    pub(crate) input: FuzzInput,
    pub(crate) failure_message: String,
}
