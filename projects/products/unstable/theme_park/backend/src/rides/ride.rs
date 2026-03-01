#![allow(dead_code)]
use crate::map::node_id::NodeId;
use crate::rides::maintenance::Maintenance;
use crate::rides::ride_id::RideId;
use crate::rides::ride_kind::RideKind;
use crate::rides::ride_queue::RideQueue;
use crate::visitors::visitor_id::VisitorId;
use serde::{Deserialize, Serialize};

/// A single attraction in the theme park.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: RideId,
    pub kind: RideKind,
    pub node: NodeId,
    pub capacity: u32,
    pub queue_capacity: u32,
    pub ticks_per_ride: u32,
    pub price: u32,
    pub queue: RideQueue,
    pub current_riders: Vec<VisitorId>,
    pub ticks_remaining: u32,
    pub running: bool,
    pub maintenance: Maintenance,
    pub total_riders_served: u32,
    pub total_revenue: u32,
}

impl Ride {
    pub fn new(
        id: RideId,
        kind: RideKind,
        node: NodeId,
        capacity: u32,
        ticks_per_ride: u32,
        price: u32,
    ) -> Self {
        Self {
            id,
            kind,
            node,
            capacity,
            queue_capacity: capacity * 4,
            ticks_per_ride,
            price,
            queue: RideQueue::new(),
            current_riders: Vec::new(),
            ticks_remaining: 0,
            running: false,
            maintenance: Maintenance::new(),
            total_riders_served: 0,
            total_revenue: 0,
        }
    }

    pub fn is_available(&self) -> bool {
        self.maintenance.is_operational()
            && self.queue.len() < self.queue_capacity as usize
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}
