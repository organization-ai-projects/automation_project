use crate::model::item_id::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub kind: String,
    pub quantity: u32,
}
