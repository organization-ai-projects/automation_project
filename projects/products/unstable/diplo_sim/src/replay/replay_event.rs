use serde::{Deserialize, Serialize};
use crate::time::turn::Turn;
use crate::orders::order_set::OrderSet;

/// A single turn's orders for replay purposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub turn: Turn,
    pub order_sets: Vec<OrderSet>,
}
