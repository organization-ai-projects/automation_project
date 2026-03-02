use serde::{Deserialize, Serialize};

use crate::war::battle_report::BattleReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnReport {
    pub turn: u64,
    pub battles: Vec<BattleReport>,
    pub diplomacy_events: Vec<String>,
    pub validation_errors: Vec<String>,
}
