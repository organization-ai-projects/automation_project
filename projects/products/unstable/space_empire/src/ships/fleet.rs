use crate::model::{EmpireId, FleetId, PlanetId};
use crate::ships::ShipKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub fleet_id: FleetId,
    pub owner: EmpireId,
    pub ships: BTreeMap<ShipKind, u32>,
    pub location: PlanetId,
}
