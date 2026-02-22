// projects/products/unstable/autonomous_dev_ai/src/neural/model_governance.rs
use super::{ConfidenceGate, DriftDetector, ModelRegistry};
use serde::{Deserialize, Serialize};

/// Top-level controller combining registry, confidence gate, and drift detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelGovernance {
    pub registry: ModelRegistry,
    pub confidence_gate: ConfidenceGate,
    pub drift_detector: DriftDetector,
    pub offline_eval_min_score: f64,
    pub online_eval_min_score: f64,
}

impl ModelGovernance {
    pub fn new() -> Self {
        Self {
            registry: ModelRegistry::new(),
            confidence_gate: ConfidenceGate::default(),
            drift_detector: DriftDetector::default(),
            offline_eval_min_score: 0.8,
            online_eval_min_score: 0.85,
        }
    }

    pub fn evaluate_offline(&mut self, model_name: &str, score: f64) -> bool {
        self.registry
            .record_offline_evaluation(model_name, score, self.offline_eval_min_score)
    }

    pub fn evaluate_online(&mut self, model_name: &str, score: f64) -> bool {
        self.registry
            .record_online_evaluation(model_name, score, self.online_eval_min_score)
    }

    pub fn promote_after_offline_gate(&mut self, model_name: &str) -> bool {
        self.registry.promote_to_canary(model_name)
    }

    pub fn promote_after_online_gate(&mut self, model_name: &str) -> bool {
        self.registry.promote_to_production(model_name)
    }

    pub fn should_fallback_to_symbolic(&self, model_name: &str, confidence: f64) -> bool {
        if !self.registry.is_serving_allowed(model_name) {
            return true;
        }
        if let Some(mv) = self.registry.get(model_name)
            && confidence < mv.confidence_threshold
        {
            return true;
        }
        !self.confidence_gate.passes(confidence)
    }

    /// Accept a model inference result: validate confidence, detect drift, auto-rollback on drift.
    ///
    /// Returns `true` if the suggestion should be used, `false` if symbolic override should take over.
    pub fn accept(&mut self, model_name: &str, confidence: f64) -> bool {
        if self.should_fallback_to_symbolic(model_name, confidence) {
            return false;
        }

        let drift_detected = self.drift_detector.observe(confidence);
        if drift_detected {
            self.registry
                .rollback(model_name, "drift detected by sliding-window monitor");
            return false;
        }
        true
    }
}

impl Default for ModelGovernance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural::{DriftDetector, ModelVersion, RolloutState};

    fn ready_governance() -> ModelGovernance {
        let mut governance = ModelGovernance::new();
        governance.registry.register(ModelVersion::new(
            "default-neural",
            "1.0.0",
            "builtin://default",
            0.7,
        ));
        let _ = governance.evaluate_offline("default-neural", 1.0);
        let _ = governance.promote_after_offline_gate("default-neural");
        let _ = governance.evaluate_online("default-neural", 1.0);
        let _ = governance.promote_after_online_gate("default-neural");
        governance
    }

    #[test]
    fn symbolic_override_remains_authoritative_on_low_confidence() {
        let mut governance = ready_governance();
        assert!(!governance.accept("default-neural", 0.2));
    }

    #[test]
    fn drift_detection_triggers_rollback_and_fallback() {
        let mut governance = ready_governance();
        governance.drift_detector = DriftDetector::new(3, 0.95);

        assert!(governance.accept("default-neural", 0.96));
        assert!(governance.accept("default-neural", 0.95));
        assert!(!governance.accept("default-neural", 0.70));
        assert_eq!(
            governance.registry.state("default-neural"),
            Some(RolloutState::RolledBack)
        );
    }

    #[test]
    fn symbolic_override_stays_authoritative_when_model_not_promoted() {
        let mut governance = ModelGovernance::new();
        governance.registry.register(ModelVersion::new(
            "default-neural",
            "1.0.0",
            "builtin://default",
            0.7,
        ));

        assert_eq!(
            governance.registry.state("default-neural"),
            Some(RolloutState::Pending)
        );
        assert!(!governance.accept("default-neural", 0.99));
    }
}
