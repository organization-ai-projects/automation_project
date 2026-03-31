use serde::{Deserialize, Serialize};

use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::supply::supplier::SupplierId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub supplier_id: SupplierId,
    pub store_id: StoreId,
    pub good: Good,
    pub quantity_per_delivery: u64,
    pub delivery_interval: u64,
    pub cost_per_unit: i64,
}
