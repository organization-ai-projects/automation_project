// projects/products/unstable/autonomous_dev_ai/src/lifecycle/rollback_manager.rs
use serde::{Deserialize, Serialize};

use crate::lifecycle::{ActionBoundary, CompensationKind};

/// Manages rollback/compensation paths for a run's action history.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollbackManager {
    pub boundaries: Vec<ActionBoundary>,
}

impl RollbackManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an action and its compensation strategy.
    pub fn record(&mut self, action_name: impl Into<String>, compensation: CompensationKind) {
        self.boundaries.push(ActionBoundary {
            action_name: action_name.into(),
            compensation,
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
    }

    /// Return all actions that require manual intervention (irreversible).
    pub fn irreversible_actions(&self) -> Vec<&ActionBoundary> {
        self.boundaries
            .iter()
            .filter(|b| matches!(b.compensation, CompensationKind::Irreversible { .. }))
            .collect()
    }

    /// Return all actions that can be automatically reversed.
    pub fn reversible_actions(&self) -> Vec<&ActionBoundary> {
        self.boundaries
            .iter()
            .filter(|b| matches!(b.compensation, CompensationKind::Reversible { .. }))
            .collect()
    }
}
