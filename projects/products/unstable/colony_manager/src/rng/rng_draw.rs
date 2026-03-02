use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngDraw {
    pub raw_value: u64,
    pub resolved_index: usize,
}
