use serde::{Deserialize, Serialize};
use super::order_id::OrderId;
use super::order_kind::OrderKind;
use crate::model::unit_id::UnitId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub unit_id: UnitId,
    pub kind: OrderKind,
}
