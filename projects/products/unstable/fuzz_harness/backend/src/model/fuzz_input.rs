use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FuzzInput {
    pub(crate) data: Vec<u8>,
    pub(crate) index: u64,
}
