use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::route::Route;
use super::star_system::StarSystem;
use super::star_system_id::StarSystemId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMap {
    pub systems: BTreeMap<StarSystemId, StarSystem>,
    pub routes: Vec<Route>,
}

impl StarMap {
    pub fn new() -> Self {
        Self {
            systems: BTreeMap::new(),
            routes: Vec::new(),
        }
    }

    /// Check if two systems are adjacent (connected by a direct route).
    pub fn are_adjacent(&self, from: &StarSystemId, to: &StarSystemId) -> bool {
        self.routes
            .iter()
            .any(|r| (&r.from == from && &r.to == to) || (&r.from == to && &r.to == from))
    }
}

impl Default for StarMap {
    fn default() -> Self {
        Self::new()
    }
}
