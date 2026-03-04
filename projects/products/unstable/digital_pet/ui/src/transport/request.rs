use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    NewRun { seed: u64, ticks: u64 },
    Step { n: u64 },
    GetReport,
}
