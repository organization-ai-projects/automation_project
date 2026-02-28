use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    TurnResolved { turn: u64 },
    TreatyFormed { treaty_id: String },
    TreatyExpired { treaty_id: String },
    BattleOccurred { location: String, attacker_wins: bool },
    EmpireDefeated { empire_id: String },
}
