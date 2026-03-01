#![allow(dead_code)]
use crate::config::sim_config::SimConfig;
use crate::economy::budget::Budget;
use crate::map::path_graph::PathGraph;
use crate::reputation::reputation::Reputation;
use crate::rides::ride::Ride;
use crate::rides::ride_id::RideId;
use crate::shops::shop::Shop;
use crate::shops::shop_id::ShopId;
use crate::time::tick_clock::TickClock;
use crate::visitors::visitor::Visitor;
use crate::visitors::visitor_id::VisitorId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Complete mutable state of the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub clock: TickClock,
    pub visitors: BTreeMap<VisitorId, Visitor>,
    pub rides: BTreeMap<RideId, Ride>,
    pub shops: BTreeMap<ShopId, Shop>,
    pub graph: PathGraph,
    pub budget: Budget,
    pub budget_initial: i64,
    pub reputation: Reputation,
    pub config: SimConfig,
}

impl SimState {
    pub fn new(
        graph: PathGraph,
        rides: BTreeMap<RideId, Ride>,
        shops: BTreeMap<ShopId, Shop>,
        initial_budget: i64,
        initial_reputation: i32,
        config: SimConfig,
    ) -> Self {
        Self {
            clock: TickClock::new(),
            visitors: BTreeMap::new(),
            rides,
            shops,
            graph,
            budget: Budget::new(initial_budget),
            budget_initial: initial_budget,
            reputation: Reputation::new(initial_reputation),
            config,
        }
    }

    pub fn add_visitor(&mut self, visitor: Visitor) {
        self.visitors.insert(visitor.id, visitor);
    }

    pub fn active_visitor_count(&self) -> usize {
        self.visitors.values().filter(|v| v.is_active()).count()
    }

    pub fn total_visitors_served(&self) -> u32 {
        self.rides.values().map(|r| r.total_riders_served).sum()
    }

    pub fn total_revenue(&self) -> u64 {
        let r: u32 = self.rides.values().map(|r| r.total_revenue).sum();
        let s: u32 = self.shops.values().map(|s| s.total_revenue).sum();
        (r + s) as u64
    }

    pub fn average_wait_ticks(&self) -> f64 {
        let completed: Vec<u32> = self
            .visitors
            .values()
            .map(|v| v.rides_completed)
            .filter(|&c| c > 0)
            .collect();
        if completed.is_empty() {
            return 0.0;
        }
        let total: u32 = completed.iter().sum();
        total as f64 / completed.len() as f64
    }
}
