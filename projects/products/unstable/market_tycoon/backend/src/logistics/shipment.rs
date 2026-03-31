use serde::{Deserialize, Serialize};

use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub origin: StoreId,
    pub destination: StoreId,
    pub good: Good,
    pub quantity: u64,
    pub depart_tick: Tick,
    pub arrive_tick: Tick,
}

impl Shipment {
    pub fn new(
        origin: StoreId,
        destination: StoreId,
        good: Good,
        quantity: u64,
        depart_tick: Tick,
        arrive_tick: Tick,
    ) -> Self {
        Self {
            origin,
            destination,
            good,
            quantity,
            depart_tick,
            arrive_tick,
        }
    }
}
