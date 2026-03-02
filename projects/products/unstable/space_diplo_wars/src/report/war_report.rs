use serde::{Deserialize, Serialize};

use crate::war::battle_report::BattleReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarReport {
    pub total_battles: usize,
    pub battles: Vec<BattleReport>,
}
