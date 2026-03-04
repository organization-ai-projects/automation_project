use crate::model::var::Var;
use crate::model::var_id::VarId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub vars: BTreeMap<String, i64>,
    pub variable_order: Vec<VarId>,
}

impl State {
    pub fn from_vars(vars: &[Var]) -> Self {
        let mut state = State::default();
        for var in vars {
            state.vars.insert(var.id.0.clone(), var.initial_value);
            state.variable_order.push(var.id.clone());
        }
        state
    }

    pub fn variable_ids(&self) -> &[VarId] {
        &self.variable_order
    }

    pub fn apply_delta(&mut self, variable_id: &VarId, delta: i64) -> i64 {
        let entry = self.vars.entry(variable_id.0.clone()).or_insert(0);
        *entry += delta;
        *entry
    }

    pub fn get(&self, variable_id: &VarId) -> Option<i64> {
        self.vars.get(&variable_id.0).copied()
    }
}
