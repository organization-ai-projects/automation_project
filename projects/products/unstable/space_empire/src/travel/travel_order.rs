use crate::model::FleetId;
use crate::time::Tick;
use crate::travel::Route;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelOrder {
    pub fleet_id: FleetId,
    pub route: Route,
    pub departure: Tick,
    pub arrival: Tick,
}
