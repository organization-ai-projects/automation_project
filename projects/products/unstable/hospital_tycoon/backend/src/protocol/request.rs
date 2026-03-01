// projects/products/unstable/hospital_tycoon/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Ping,
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
