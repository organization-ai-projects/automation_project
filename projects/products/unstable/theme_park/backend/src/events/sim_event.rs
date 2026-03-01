#![allow(dead_code)]
use crate::rides::ride_id::RideId;
use crate::shops::shop_id::ShopId;
use crate::time::tick::Tick;
use crate::visitors::visitor_id::VisitorId;
use serde::{Deserialize, Serialize};

/// Every domain event emitted during the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEvent {
    VisitorEntered {
        tick: Tick,
        visitor_id: VisitorId,
    },
    VisitorLeft {
        tick: Tick,
        visitor_id: VisitorId,
        mood: i32,
    },
    VisitorJoinedQueue {
        tick: Tick,
        visitor_id: VisitorId,
        ride_id: RideId,
    },
    VisitorExitedRide {
        tick: Tick,
        visitor_id: VisitorId,
        ride_id: RideId,
    },
    VisitorShopVisit {
        tick: Tick,
        visitor_id: VisitorId,
        shop_id: ShopId,
        spent: u32,
    },
    RideStarted {
        tick: Tick,
        ride_id: RideId,
        riders_count: u32,
    },
    RideComplete {
        tick: Tick,
        ride_id: RideId,
        riders_count: u32,
    },
    MaintenanceStarted {
        tick: Tick,
        ride_id: RideId,
    },
    MaintenanceComplete {
        tick: Tick,
        ride_id: RideId,
    },
}

impl SimEvent {
    pub fn tick(&self) -> Tick {
        match self {
            SimEvent::VisitorEntered { tick, .. }
            | SimEvent::VisitorLeft { tick, .. }
            | SimEvent::VisitorJoinedQueue { tick, .. }
            | SimEvent::VisitorExitedRide { tick, .. }
            | SimEvent::VisitorShopVisit { tick, .. }
            | SimEvent::RideStarted { tick, .. }
            | SimEvent::RideComplete { tick, .. }
            | SimEvent::MaintenanceStarted { tick, .. }
            | SimEvent::MaintenanceComplete { tick, .. } => *tick,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            SimEvent::VisitorEntered { .. } => "VisitorEntered",
            SimEvent::VisitorLeft { .. } => "VisitorLeft",
            SimEvent::VisitorJoinedQueue { .. } => "VisitorJoinedQueue",
            SimEvent::VisitorExitedRide { .. } => "VisitorExitedRide",
            SimEvent::VisitorShopVisit { .. } => "VisitorShopVisit",
            SimEvent::RideStarted { .. } => "RideStarted",
            SimEvent::RideComplete { .. } => "RideComplete",
            SimEvent::MaintenanceStarted { .. } => "MaintenanceStarted",
            SimEvent::MaintenanceComplete { .. } => "MaintenanceComplete",
        }
    }
}
