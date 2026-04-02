use crate::model::item_id::ItemId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: BTreeMap<String, u32>,
}

impl Inventory {
    pub fn add(&mut self, item_id: &ItemId, count: u32) {
        let entry = self.items.entry(item_id.0.clone()).or_insert(0);
        *entry += count;
    }

    pub fn use_item(&mut self, item_id: &ItemId) -> bool {
        if let Some(count) = self.items.get_mut(&item_id.0) {
            if *count > 0 {
                *count -= 1;
                return true;
            }
        }
        false
    }

    pub fn count(&self, item_id: &ItemId) -> u32 {
        self.items.get(&item_id.0).copied().unwrap_or(0)
    }
}
