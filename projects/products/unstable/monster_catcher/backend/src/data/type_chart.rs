use crate::data::type_id::TypeId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeChart {
    pub effectiveness: BTreeMap<String, f64>,
}

impl Default for TypeChart {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChart {
    pub fn new() -> Self {
        Self {
            effectiveness: BTreeMap::new(),
        }
    }

    pub fn key(attacker: &TypeId, defender: &TypeId) -> String {
        format!("{}>{}", attacker.0, defender.0)
    }

    pub fn set(&mut self, attacker: &TypeId, defender: &TypeId, factor: f64) {
        self.effectiveness
            .insert(Self::key(attacker, defender), factor);
    }

    pub fn get(&self, attacker: &TypeId, defender: &TypeId) -> f64 {
        self.effectiveness
            .get(&Self::key(attacker, defender))
            .copied()
            .unwrap_or(1.0)
    }

    pub fn compute_effectiveness(&self, attack_type: &TypeId, defender_types: &[TypeId]) -> f64 {
        let mut factor = 1.0;
        for def_type in defender_types {
            factor *= self.get(attack_type, def_type);
        }
        factor
    }
}
