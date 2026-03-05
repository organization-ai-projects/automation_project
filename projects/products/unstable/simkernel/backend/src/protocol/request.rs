use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Ping,
    ListPacks,
    LoadScenario {
        path: String,
    },
    ValidateScenario {
        path: String,
    },
    NewRun {
        pack_kind: String,
        seed: f64,
        ticks: f64,
        turns: f64,
        ticks_per_turn: f64,
    },
    SubmitCommands {
        commands: Vec<String>,
    },
    Step {
        n_ticks: f64,
    },
    RunToEnd,
    GetSnapshot {
        at_tick: f64,
        at_turn: f64,
    },
    Query {
        query: String,
    },
    GetReport,
    SaveReplay {
        path: String,
    },
    LoadReplay {
        path: String,
    },
    ReplayToEnd,
    Shutdown,
}
