use crate::combat::{CombatInput, CombatRound};
use crate::model::EmpireId;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReport {
    pub input: CombatInput,
    pub rounds: Vec<CombatRound>,
    pub winner: Option<EmpireId>,
    pub tick: Tick,
}

#[allow(dead_code)]
impl BattleReport {
    pub fn is_attacker_victor(&self) -> bool {
        self.winner == Some(self.input.attacker_empire)
    }
}
