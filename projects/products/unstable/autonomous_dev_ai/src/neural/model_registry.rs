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
    /// Offline evaluation gate result per model.
    offline_eval_passed: HashMap<String, bool>,
    /// Online evaluation gate result per model.
    online_eval_passed: HashMap<String, bool>,
    /// Last rollback reason per model.
    rollback_reasons: HashMap<String, String>,
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
        self.offline_eval_passed.insert(name.clone(), false);
        self.online_eval_passed.insert(name.clone(), false);
        self.rollback_reasons.remove(&name);
        self.models.insert(name, version);
    }

    pub fn record_offline_evaluation(
        &mut self,
        name: &str,
        observed_score: f64,
        min_required_score: f64,
    ) -> bool {
        let passed = observed_score >= min_required_score;
        self.offline_eval_passed.insert(name.to_string(), passed);
        passed
    }

    pub fn record_online_evaluation(
        &mut self,
        name: &str,
        observed_score: f64,
        min_required_score: f64,
    ) -> bool {
        let passed = observed_score >= min_required_score;
        self.online_eval_passed.insert(name.to_string(), passed);
        passed
    }

    pub fn offline_gate_passed(&self, name: &str) -> bool {
        self.offline_eval_passed.get(name).copied().unwrap_or(false)
    }

    pub fn online_gate_passed(&self, name: &str) -> bool {
        self.online_eval_passed.get(name).copied().unwrap_or(false)
    }

    /// Advance a model from Pending -> Canary after passing offline evaluation.
    pub fn promote_to_canary(&mut self, name: &str) -> bool {
        if self.rollout_states.get(name) == Some(&RolloutState::Pending)
            && self.offline_gate_passed(name)
        {
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
        if self.rollout_states.get(name) == Some(&RolloutState::Canary)
            && self.online_gate_passed(name)
        {
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
        self.rollback_reasons
            .insert(name.to_string(), reason.to_string());
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

    pub fn is_serving_allowed(&self, name: &str) -> bool {
        let Some(model) = self.models.get(name) else {
            return false;
        };
        if !model.active {
            return false;
        }
        matches!(
            self.rollout_states.get(name),
            Some(RolloutState::Canary | RolloutState::Production)
        )
    }

    pub fn rollback_reason(&self, name: &str) -> Option<&str> {
        self.rollback_reasons.get(name).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollout_requires_offline_and_online_gates() {
        let mut registry = ModelRegistry::new();
        registry.register(ModelVersion::new("m", "1.0.0", "builtin://m", 0.7));

        assert!(!registry.promote_to_canary("m"));
        assert!(registry.record_offline_evaluation("m", 0.91, 0.8));
        assert!(registry.promote_to_canary("m"));
        assert_eq!(registry.state("m"), Some(RolloutState::Canary));

        assert!(!registry.promote_to_production("m"));
        assert!(registry.record_online_evaluation("m", 0.93, 0.85));
        assert!(registry.promote_to_production("m"));
        assert_eq!(registry.state("m"), Some(RolloutState::Production));
    }

    #[test]
    fn rollback_disables_serving_and_records_reason() {
        let mut registry = ModelRegistry::new();
        registry.register(ModelVersion::new("m", "1.0.0", "builtin://m", 0.7));
        let _ = registry.record_offline_evaluation("m", 1.0, 0.8);
        let _ = registry.promote_to_canary("m");

        assert!(registry.is_serving_allowed("m"));
        registry.rollback("m", "drift");
        assert_eq!(registry.state("m"), Some(RolloutState::RolledBack));
        assert_eq!(registry.rollback_reason("m"), Some("drift"));
        assert!(!registry.is_serving_allowed("m"));
    }
}
