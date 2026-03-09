use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Request {
    RunSimulation {
        seed: u64,
        days: u32,
    },
    ReplaySimulation {
        replay_path: String,
    },
    ExportReport {
        format: String,
        seed: u64,
        days: u32,
    },
}
