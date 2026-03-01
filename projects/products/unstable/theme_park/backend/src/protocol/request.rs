#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// All request kinds understood by the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Ping,
    Shutdown,
    LoadScenario { path: String },
    NewRun { seed: u64, ticks: u64 },
    Step { n: u64 },
    RunToEnd,
    GetSnapshot { at_tick: u64 },
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}
