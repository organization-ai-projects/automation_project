use crate::model::item::Item;
use crate::model::item_id::ItemId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: BTreeMap<ItemId, Item>,
}

impl Inventory {
    pub fn add(&mut self, item: Item) { self.items.insert(item.id, item); }
    pub fn remove(&mut self, id: &ItemId) -> Option<Item> { self.items.remove(id) }
}
