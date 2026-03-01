#![allow(dead_code)]
use crate::map::node_id::NodeId;
use crate::shops::shop_id::ShopId;
use serde::{Deserialize, Serialize};

/// A retail shop in the park.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    pub id: ShopId,
    pub node: NodeId,
    pub name: String,
    pub price: u32,
    pub total_revenue: u32,
    pub total_customers: u32,
}

impl Shop {
    pub fn new(id: ShopId, node: NodeId, name: impl Into<String>, price: u32) -> Self {
        Self {
            id,
            node,
            name: name.into(),
            price,
            total_revenue: 0,
            total_customers: 0,
        }
    }
}
