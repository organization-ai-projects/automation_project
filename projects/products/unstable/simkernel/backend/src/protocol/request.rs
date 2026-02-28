#![allow(dead_code)]
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
        seed: u64,
        ticks: u64,
        turns: u64,
        ticks_per_turn: u64,
    },
    SubmitCommands {
        commands: Vec<String>,
    },
    Step {
        n_ticks: u64,
    },
    RunToEnd,
    GetSnapshot {
        at_tick: u64,
        at_turn: u64,
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
