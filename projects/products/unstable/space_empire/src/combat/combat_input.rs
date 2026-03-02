use crate::model::{EmpireId, PlanetId};
use crate::ships::FleetComposition;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatInput {
    pub attacker_empire: EmpireId,
    pub defender_empire: EmpireId,
    pub location: PlanetId,
    pub tick: Tick,
    pub attacker_fleet: FleetComposition,
    pub defender_fleet: FleetComposition,
}
