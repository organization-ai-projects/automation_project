use serde::{Deserialize, Serialize};
use super::territory_id::TerritoryId;
use super::territory::Territory;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapGraph {
    pub name: String,
    pub version: String,
    pub territories: Vec<Territory>,
    /// Each adjacency is a pair [a, b] meaning territory a and b are adjacent (bidirectional).
    pub adjacencies: Vec<[TerritoryId; 2]>,
}

impl MapGraph {
    pub fn is_adjacent(&self, a: TerritoryId, b: TerritoryId) -> bool {
        self.adjacencies
            .iter()
            .any(|edge| (edge[0] == a && edge[1] == b) || (edge[0] == b && edge[1] == a))
    }

    pub fn neighbors(&self, t: TerritoryId) -> Vec<TerritoryId> {
        let mut result = Vec::new();
        for edge in &self.adjacencies {
            if edge[0] == t {
                result.push(edge[1]);
            } else if edge[1] == t {
                result.push(edge[0]);
            }
        }
        result.sort();
        result
    }

    pub fn territory_exists(&self, id: TerritoryId) -> bool {
        self.territories.iter().any(|t| t.id == id)
    }

    pub fn territory_count(&self) -> usize {
        self.territories.len()
    }
}
