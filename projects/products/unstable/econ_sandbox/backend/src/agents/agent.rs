use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::agents::agent_id::AgentId;
use crate::market::good::Good;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub cash: i64,
    pub inventory: BTreeMap<Good, u64>,
    pub production: BTreeMap<Good, u64>,
    pub consumption: BTreeMap<Good, u64>,
}

impl Agent {
    pub fn new(id: AgentId, name: String, cash: i64) -> Self {
        Self {
            id,
            name,
            cash,
            inventory: BTreeMap::new(),
            production: BTreeMap::new(),
            consumption: BTreeMap::new(),
        }
    }

    pub fn produce(&mut self) {
        for (&good, &amount) in &self.production {
            *self.inventory.entry(good).or_insert(0) += amount;
        }
    }

    pub fn consume(&mut self) {
        for (&good, &amount) in &self.consumption {
            let held = self.inventory.entry(good).or_insert(0);
            if *held >= amount {
                *held -= amount;
            }
        }
    }
}
