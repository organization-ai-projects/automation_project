use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::tech_kind::TechKind;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechTree {
    /// Tech level per kind, starts at 0.
    pub levels: BTreeMap<TechKind, u32>,
}

impl TechTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(&self, kind: TechKind) -> u32 {
        *self.levels.get(&kind).unwrap_or(&0)
    }

    pub fn advance(&mut self, kind: TechKind) {
        *self.levels.entry(kind).or_insert(0) += 1;
    }
}
