use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;

use super::ship_kind::ShipKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub id: FleetId,
    pub empire_id: EmpireId,
    /// Ship counts by kind.
    pub ships: BTreeMap<ShipKind, u32>,
    pub location: String,
    pub destination: Option<String>,
}

impl Fleet {
    /// Compute total attack power: sum of (ship_count * ship_attack).
    pub fn attack_power(&self) -> i64 {
        use super::ship_stats::ShipStats;
        self.ships
            .iter()
            .map(|(kind, count)| ShipStats::for_kind(*kind).attack * (*count as i64))
            .sum()
    }

    /// Compute total defense power: sum of (ship_count * ship_defense).
    pub fn defense_power(&self) -> i64 {
        use super::ship_stats::ShipStats;
        self.ships
            .iter()
            .map(|(kind, count)| ShipStats::for_kind(*kind).defense * (*count as i64))
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.ships.values().all(|c| *c == 0)
    }
}
