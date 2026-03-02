use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FleetOrder {
    Move {
        destination: String,
    },
    Attack {
        target_fleet: String,
        system: String,
    },
    Defend {
        system: String,
    },
    Patrol {
        system: String,
    },
}
