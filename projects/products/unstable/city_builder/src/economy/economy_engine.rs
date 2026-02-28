use super::TaxPolicy;
use crate::config::sim_config::SimConfig;
use crate::snapshot::state_snapshot::StateSnapshot;

#[derive(Debug, Clone)]
pub struct EconomyEngine;

impl EconomyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn tick(&mut self, state: &mut StateSnapshot, _config: &SimConfig) {
        let mut income = 0i64;
        let buildings: Vec<_> = state.buildings.values().cloned().collect();
        for b in &buildings {
            income += TaxPolicy::tax_per_building(b.zone);
        }
        state.budget_balance += income;
    }
}

impl Default for EconomyEngine {
    fn default() -> Self {
        Self::new()
    }
}
