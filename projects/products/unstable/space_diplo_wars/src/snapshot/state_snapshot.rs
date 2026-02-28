use serde::{Deserialize, Serialize};

use crate::model::sim_state::SimState;

/// A serialisable point-in-time snapshot of the game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub game_id: String,
    pub current_tick: u64,
    pub current_turn: u64,
    pub empire_count: usize,
    pub fleet_count: usize,
    pub treaty_count: usize,
    /// Sorted empire resource totals for canonical comparison.
    pub empire_resources:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, i64>>,
}

impl StateSnapshot {
    pub fn from_state(state: &SimState) -> Self {
        let empire_resources = state
            .empires
            .iter()
            .map(|(eid, emp)| {
                let resources: std::collections::BTreeMap<String, i64> = emp
                    .resources
                    .amounts
                    .iter()
                    .map(|(k, v)| (format!("{:?}", k), *v))
                    .collect();
                (eid.0.clone(), resources)
            })
            .collect();

        Self {
            game_id: state.game_id.0.clone(),
            current_tick: state.current_tick.0,
            current_turn: state.current_turn.0,
            empire_count: state.empires.len(),
            fleet_count: state.fleets.len(),
            treaty_count: state.treaties.len(),
            empire_resources,
        }
    }
}
