#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// UI application state (no business logic).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub scenario_path: Option<String>,
    pub seed: u64,
    pub ticks: u64,
    pub last_report: Option<String>,
    pub last_error: Option<String>,
    pub run_complete: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
