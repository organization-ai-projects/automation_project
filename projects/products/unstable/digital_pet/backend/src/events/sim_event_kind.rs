use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEventKind {
    Evolved { from: String, to: String },
    CareAction { kind: String },
    CareMistake { reason: String },
    BattleStarted,
    BattleEnded { winner: String },
}
