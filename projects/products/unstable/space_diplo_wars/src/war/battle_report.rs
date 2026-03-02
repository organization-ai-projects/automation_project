use serde::{Deserialize, Serialize};

use crate::map::star_system_id::StarSystemId;
use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;

/// Canonical battle outcome (ordered by attacker_fleet_id for determinism).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReport {
    pub location: StarSystemId,
    pub attacker_empire: EmpireId,
    pub attacker_fleet: FleetId,
    pub attacker_power: i64,
    pub defender_empire: EmpireId,
    pub defender_fleet: FleetId,
    pub defender_power: i64,
    /// true = attacker wins; false = defender wins (tie goes to defender).
    pub attacker_wins: bool,
}
