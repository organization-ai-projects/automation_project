// projects/products/unstable/hospital_tycoon/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Ping,
    LoadScenario { path: String },
    NewRun { seed: f64, ticks: f64 },
    Step { n: f64 },
    RunToEnd,
    GetSnapshot { at_tick: f64 },
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}
