#![allow(dead_code)]
use crate::map::node_id::NodeId;
use crate::rides::ride_id::RideId;
use crate::shops::shop_id::ShopId;
use crate::visitors::mood::Mood;
use crate::visitors::patience::Patience;
use crate::visitors::preference::Preference;
use crate::visitors::visitor_id::VisitorId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visitor {
    pub id: VisitorId,
    pub current_node: NodeId,
    pub mood: Mood,
    pub patience: Patience,
    pub preference: Preference,
    pub status: VisitorStatus,
    pub rides_completed: u32,
    pub revenue_generated: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisitorStatus {
    Idle,
    Walking { path: Vec<NodeId>, step: usize },
    Queued(RideId),
    Riding(RideId),
    Shopping { shop: ShopId, ticks_remaining: u32 },
    Left,
}

impl Visitor {
    pub fn new(id: VisitorId, start_node: NodeId, preference: Preference) -> Self {
        Self {
            id,
            current_node: start_node,
            mood: Mood::new(Mood::INITIAL),
            patience: Patience::new(Patience::INITIAL),
            preference,
            status: VisitorStatus::Idle,
            rides_completed: 0,
            revenue_generated: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        !matches!(self.status, VisitorStatus::Left)
    }
}
