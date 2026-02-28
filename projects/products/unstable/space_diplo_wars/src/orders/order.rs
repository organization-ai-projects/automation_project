use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::empire_id::EmpireId;

use super::order_id::OrderId;
use super::order_kind::OrderKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub empire_id: EmpireId,
    pub kind: OrderKind,
    /// Flexible parameters for the order.
    pub params: BTreeMap<String, String>,
}
