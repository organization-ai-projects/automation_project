use serde::{Deserialize, Serialize};

use super::order::Order;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSet {
    pub turn: u64,
    pub orders: Vec<Order>,
}

impl OrderSet {
    pub fn new(turn: u64) -> Self {
        Self { turn, orders: Vec::new() }
    }
}
