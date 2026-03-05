use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::resource_kind::ResourceKind;

/// Per-tick resource output for an empire's systems.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Production {
    pub output: BTreeMap<ResourceKind, i64>,
}

impl Production {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, kind: ResourceKind, amount: i64) {
        *self.output.entry(kind).or_insert(0) += amount;
    }
}
