// projects/products/unstable/digital_pet/backend/src/replay/replay_file.rs
use crate::care::care_action::CareAction;
use crate::scenario::scenario::Scenario;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub ticks: u64,
    pub scenario: Scenario,
    pub actions: Vec<CareAction>,
}

impl ReplayFile {
    pub fn new(seed: u64, ticks: u64, scenario: Scenario) -> Self {
        Self { seed, ticks, scenario, actions: vec![] }
    }
}
