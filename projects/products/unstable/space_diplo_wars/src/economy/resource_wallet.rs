use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::resource_kind::ResourceKind;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceWallet {
    pub amounts: BTreeMap<ResourceKind, i64>,
}

impl ResourceWallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, kind: ResourceKind) -> i64 {
        *self.amounts.get(&kind).unwrap_or(&0)
    }

    pub fn add(&mut self, kind: ResourceKind, amount: i64) {
        *self.amounts.entry(kind).or_insert(0) += amount;
    }

    pub fn spend(&mut self, kind: ResourceKind, amount: i64) -> bool {
        let current = self.get(kind);
        if current >= amount {
            *self.amounts.entry(kind).or_insert(0) -= amount;
            true
        } else {
            false
        }
    }
}
