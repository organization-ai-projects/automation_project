// projects/products/unstable/hospital_tycoon/ui/src/app/app_state.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub seed: u64,
    pub ticks: u64,
    pub current_tick: u64,
    pub running: bool,
    pub patients_treated: u32,
    pub final_budget: i64,
    pub reputation: u32,
    pub last_event: Option<String>,
    pub run_hash: Option<String>,
}

impl AppState {
    pub fn new(seed: u64, ticks: u64) -> Self {
        Self {
            seed,
            ticks,
            current_tick: 0,
            running: false,
            patients_treated: 0,
            final_budget: 0,
            reputation: 0,
            last_event: None,
            run_hash: None,
        }
    }
}
