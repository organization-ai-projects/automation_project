use crate::rng::rng_draw::RngDraw;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event_type", rename_all = "camelCase")]
pub enum GameEvent {
    RunStarted {
        seed: u64,
    },
    ScenarioLoaded {
        name: String,
    },
    EncounterGenerated {
        species: String,
        level: u32,
    },
    CaptureAttempted {
        success: bool,
        roll: u64,
        threshold: u64,
    },
    BattleStarted {
        battle_id: String,
    },
    BattleTurn {
        turn_number: u32,
    },
    BattleEnded {
        player_won: bool,
    },
    XpAwarded {
        amount: u64,
        new_level: u32,
    },
    MoveUnlocked {
        move_id: String,
    },
    RngDraw {
        draw: RngDraw,
    },
    SnapshotTaken {
        hash: String,
    },
}
