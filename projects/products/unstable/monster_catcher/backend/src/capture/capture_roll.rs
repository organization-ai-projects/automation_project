use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CaptureRoll {
    pub step: u64,
    pub capture_rate: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    pub roll: u64,
    pub threshold: u64,
    pub success: bool,
}
