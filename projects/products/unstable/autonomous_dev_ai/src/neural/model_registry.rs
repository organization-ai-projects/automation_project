// projects/products/unstable/autonomous_dev_ai/src/neural/model_registry.rs
use super::{ModelVersion, RolloutState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry that pins model versions and controls rollout lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelRegistry {
    /// Active model versions by logical name.
    models: HashMap<String, ModelVersion>,
    /// Rollout state per model name.
    rollout_states: HashMap<String, RolloutState>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a model version (initially in Pending state).
    pub fn register(&mut self, version: ModelVersion) {
        let name = version.name.clone();
        self.rollout_states
            .entry(name.clone())
            .or_insert(RolloutState::Pending);
        self.models.insert(name, version);
    }

    /// Advance a model from Pending -> Canary after passing offline evaluation.
    pub fn promote_to_canary(&mut self, name: &str) -> bool {
        if self.rollout_states.get(name) == Some(&RolloutState::Pending) {
            self.rollout_states
                .insert(name.to_string(), RolloutState::Canary);
            if let Some(m) = self.models.get_mut(name) {
                m.active = true;
            }
            return true;
        }
        false
    }

    /// Advance a model from Canary -> Production after online evaluation passes.
    pub fn promote_to_production(&mut self, name: &str) -> bool {
        if self.rollout_states.get(name) == Some(&RolloutState::Canary) {
            self.rollout_states
                .insert(name.to_string(), RolloutState::Production);
            return true;
        }
        false
    }

    /// Roll back a model to RolledBack state (e.g., on drift detection).
    pub fn rollback(&mut self, name: &str, reason: &str) {
        tracing::warn!("Model '{}' rolled back: {}", name, reason);
        self.rollout_states
            .insert(name.to_string(), RolloutState::RolledBack);
        if let Some(m) = self.models.get_mut(name) {
            m.active = false;
        }
    }

    pub fn state(&self, name: &str) -> Option<RolloutState> {
        self.rollout_states.get(name).copied()
    }

    pub fn get(&self, name: &str) -> Option<&ModelVersion> {
        self.models.get(name)
    }
}
