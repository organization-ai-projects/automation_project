#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SystemStage {
    PreTick = 0,
    Tick = 1,
    PostTick = 2,
    TurnBoundary = 3,
    Report = 4,
}
