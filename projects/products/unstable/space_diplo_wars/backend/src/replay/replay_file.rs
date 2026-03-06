use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::orders::order_set::OrderSet;
use crate::scenarios::scenario::Scenario;

/// Serialisable record of a full game run, for deterministic replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub ticks_per_turn: u64,
    pub scenario_hash: String,
    pub scenario: Scenario,
    /// Orders submitted per turn (keyed by turn number as string for JSON compat).
    pub orders_per_turn: BTreeMap<String, OrderSet>,
    /// Treaty decisions per turn.
    pub treaty_decisions: BTreeMap<String, BTreeMap<String, String>>,
}

impl ReplayFile {
    pub fn new(seed: u64, ticks_per_turn: u64, scenario_hash: String, scenario: Scenario) -> Self {
        Self {
            seed,
            ticks_per_turn,
            scenario_hash,
            scenario,
            orders_per_turn: BTreeMap::new(),
            treaty_decisions: BTreeMap::new(),
        }
    }
}
