// projects/products/unstable/digital_pet/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    LoadScenario { path: String },
    NewRun { seed: u64, ticks: u64 },
    Step { n: u64 },
    CareAction { kind: String },
    Training { kind: String },
    StartBattle,
    BattleStep,
    GetSnapshot,
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}
