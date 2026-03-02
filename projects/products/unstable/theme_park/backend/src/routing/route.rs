#![allow(dead_code)]
use crate::map::node_id::NodeId;
use serde::{Deserialize, Serialize};

/// A hop-by-hop path through the park map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub steps: Vec<NodeId>,
}

impl Route {
    pub fn new(steps: Vec<NodeId>) -> Self {
        Self { steps }
    }

    pub fn empty() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
