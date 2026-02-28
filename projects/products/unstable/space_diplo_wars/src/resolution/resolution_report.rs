use serde::{Deserialize, Serialize};

use crate::events::game_event::GameEvent;
use crate::war::battle_report::BattleReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionReport {
    pub turn: u64,
    pub validation_errors: Vec<String>,
    pub diplomacy_events: Vec<String>,
    pub battles: Vec<BattleReport>,
    pub economy_events: Vec<String>,
    pub game_events: Vec<GameEvent>,
}

impl ResolutionReport {
    pub fn new(turn: u64) -> Self {
        Self {
            turn,
            validation_errors: Vec::new(),
            diplomacy_events: Vec::new(),
            battles: Vec::new(),
            economy_events: Vec::new(),
            game_events: Vec::new(),
        }
    }
}
