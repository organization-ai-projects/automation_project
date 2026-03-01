#![allow(dead_code)]
use crate::replay::replay_file::ReplayFile;
use crate::rides::ride_kind::RideKind;
use serde::{Deserialize, Serialize};

/// A node definition in the scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDef {
    pub id: u32,
    pub name: String,
}

/// An edge definition in the scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDef {
    pub from: u32,
    pub to: u32,
    pub cost: u32,
}

/// A ride definition in the scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideDef {
    pub id: u32,
    pub kind: RideKind,
    pub node: u32,
    pub capacity: u32,
    pub ticks_per_ride: u32,
    pub price: u32,
}

/// A shop definition in the scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopDef {
    pub id: u32,
    pub node: u32,
    pub name: String,
    pub price: u32,
}

/// A complete scenario configuration loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub description: String,
    pub seed: u64,
    pub visitor_count: u32,
    pub entrance_node: u32,
    pub nodes: Vec<NodeDef>,
    pub edges: Vec<EdgeDef>,
    pub rides: Vec<RideDef>,
    pub shops: Vec<ShopDef>,
    pub initial_budget: u32,
    pub initial_reputation: u32,
}

impl Scenario {
    pub fn hash(&self) -> String {
        use sha2::{Digest, Sha256};
        // Canonical hash uses sorted JSON fields (serde preserves declaration order).
        let data = serde_json::to_string(self).unwrap_or_default();
        let mut h = Sha256::new();
        h.update(data.as_bytes());
        hex::encode(h.finalize())
    }

    /// Reconstruct a minimal Scenario from a ReplayFile's stored scenario JSON.
    pub fn from_replay(replay: &ReplayFile) -> Self {
        serde_json::from_str(&replay.scenario_json).unwrap_or_else(|_| Self::default_scenario())
    }

    fn default_scenario() -> Self {
        Self {
            id: String::from("default"),
            description: String::from("Default scenario"),
            seed: 0,
            visitor_count: 5,
            entrance_node: 0,
            nodes: vec![
                NodeDef { id: 0, name: String::from("entrance") },
                NodeDef { id: 1, name: String::from("ride_area") },
            ],
            edges: vec![EdgeDef { from: 0, to: 1, cost: 1 }],
            rides: vec![RideDef {
                id: 0,
                kind: RideKind::Coaster,
                node: 1,
                capacity: 4,
                ticks_per_ride: 5,
                price: 10,
            }],
            shops: vec![],
            initial_budget: 1000,
            initial_reputation: 50,
        }
    }
}
