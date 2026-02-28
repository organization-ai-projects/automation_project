use serde::{Deserialize, Serialize};
use super::order::Order;
use super::order_id::OrderId;
use crate::model::faction_id::FactionId;
use crate::model::unit_id::UnitId;
use crate::orders::order_kind::OrderKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderSet {
    pub faction_id: FactionId,
    pub orders: Vec<Order>,
}

impl OrderSet {
    pub fn new(faction_id: FactionId, orders: Vec<Order>) -> Self {
        Self { faction_id, orders }
    }

    /// Build an OrderSet from raw (unit_id, kind) pairs, assigning sequential OrderIds.
    pub fn from_raw(faction_id: FactionId, raw: Vec<(UnitId, OrderKind)>, next_order_id: &mut u32) -> Self {
        let orders = raw
            .into_iter()
            .map(|(unit_id, kind)| {
                let id = OrderId(*next_order_id);
                *next_order_id += 1;
                Order { id, unit_id, kind }
            })
            .collect();
        Self { faction_id, orders }
    }
}
