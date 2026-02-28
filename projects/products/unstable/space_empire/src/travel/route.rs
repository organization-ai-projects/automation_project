use crate::model::PlanetId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub from: PlanetId,
    pub to: PlanetId,
}

impl Route {
    pub fn travel_distance(&self) -> u64 {
        self.from.0.abs_diff(self.to.0) as u64 * 1000 + 1000
    }
}
