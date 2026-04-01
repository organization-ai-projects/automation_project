use crate::model::FuzzInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FailureRecord {
    pub(crate) input: FuzzInput,
    pub(crate) message: String,
}
