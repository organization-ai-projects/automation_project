#![allow(dead_code)]

/// All actions that can be dispatched in the UI.
#[derive(Debug, Clone)]
pub enum Action {
    SetScenario(String),
    SetSeed(u64),
    SetTicks(u64),
    RunComplete { report_json: String },
    Error(String),
    Reset,
}
