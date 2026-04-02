use serde::{Deserialize, Serialize};

use crate::agents::agent_id::AgentId;
use crate::market::good::Good;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub agent_id: AgentId,
    pub good: Good,
    pub side: OrderSide,
    pub price: i64,
    pub quantity: u64,
}
