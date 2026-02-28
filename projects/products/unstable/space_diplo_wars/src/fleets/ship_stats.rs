use serde::{Deserialize, Serialize};

/// Combat and movement statistics for a ship class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStats {
    pub attack: i64,
    pub defense: i64,
    pub speed: u32,
    pub cost_metal: i64,
    pub cost_energy: i64,
}

impl ShipStats {
    pub fn for_kind(kind: super::ship_kind::ShipKind) -> Self {
        use super::ship_kind::ShipKind::*;
        match kind {
            Fighter => Self {
                attack: 2,
                defense: 1,
                speed: 3,
                cost_metal: 10,
                cost_energy: 5,
            },
            Cruiser => Self {
                attack: 5,
                defense: 4,
                speed: 2,
                cost_metal: 30,
                cost_energy: 15,
            },
            Battleship => Self {
                attack: 10,
                defense: 8,
                speed: 1,
                cost_metal: 80,
                cost_energy: 40,
            },
            Transport => Self {
                attack: 0,
                defense: 1,
                speed: 2,
                cost_metal: 20,
                cost_energy: 10,
            },
            Scout => Self {
                attack: 1,
                defense: 0,
                speed: 4,
                cost_metal: 5,
                cost_energy: 3,
            },
        }
    }
}
