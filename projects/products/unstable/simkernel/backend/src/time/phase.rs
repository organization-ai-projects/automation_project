#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    PreTick,
    Tick,
    PostTick,
    TurnBoundary,
    Report,
}
