use crate::ships::ShipKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetComposition {
    pub ships: BTreeMap<ShipKind, u32>,
}

impl FleetComposition {
    pub fn total_ships(&self) -> u32 {
        self.ships.values().sum()
    }

    pub fn is_empty(&self) -> bool {
        self.ships.values().all(|&c| c == 0)
    }
}
