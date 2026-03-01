use crate::model::item::Item;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A sorted map from item name to count, representing a stockpile.
/// BTreeMap ensures deterministic serialization order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    counts: BTreeMap<String, u64>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `amount` of `item` to the inventory.
    pub fn add(&mut self, item: &Item, amount: u64) {
        *self.counts.entry(item.name.clone()).or_insert(0) += amount;
    }

    /// Removes `amount` of `item` from the inventory.
    /// Returns `true` on success, `false` if insufficient stock.
    pub fn remove(&mut self, item: &Item, amount: u64) -> bool {
        let entry = self.counts.entry(item.name.clone()).or_insert(0);
        if *entry >= amount {
            *entry -= amount;
            true
        } else {
            false
        }
    }

    /// Returns the current count for an item.
    pub fn count(&self, item: &Item) -> u64 {
        self.counts.get(&item.name).copied().unwrap_or(0)
    }

    /// Returns the total number of items across all types.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }

    /// Returns the underlying sorted counts map.
    pub fn counts(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iron() -> Item {
        Item::new("iron")
    }

    #[test]
    fn add_and_count() {
        let mut inv = Inventory::new();
        inv.add(&iron(), 5);
        assert_eq!(inv.count(&iron()), 5);
    }

    #[test]
    fn remove_success() {
        let mut inv = Inventory::new();
        inv.add(&iron(), 3);
        assert!(inv.remove(&iron(), 2));
        assert_eq!(inv.count(&iron()), 1);
    }

    #[test]
    fn remove_insufficient() {
        let mut inv = Inventory::new();
        inv.add(&iron(), 1);
        assert!(!inv.remove(&iron(), 5));
        assert_eq!(inv.count(&iron()), 1);
    }
}
