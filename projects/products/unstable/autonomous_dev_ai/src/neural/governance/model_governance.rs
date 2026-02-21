use super::{ConfidenceGate, DriftDetector, ModelRegistry};
use serde::{Deserialize, Serialize};

/// Top-level controller combining registry, confidence gate, and drift detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelGovernance {
    pub registry: ModelRegistry,
    pub confidence_gate: ConfidenceGate,
    pub drift_detector: DriftDetector,
}

impl ModelGovernance {
    pub fn new() -> Self {
        Self {
            registry: ModelRegistry::new(),
            confidence_gate: ConfidenceGate::default(),
            drift_detector: DriftDetector::default(),
        }
    }

    /// Accept a model inference result: validate confidence, detect drift, auto-rollback on drift.
    ///
    /// Returns `true` if the suggestion should be used, `false` if symbolic override should take over.
    pub fn accept(&mut self, model_name: &str, confidence: f64) -> bool {
        let drift_detected = self.drift_detector.observe(confidence);
        if drift_detected {
            self.registry
                .rollback(model_name, "drift detected by sliding-window monitor");
            return false;
        }

        // Check per-model threshold
        if let Some(mv) = self.registry.get(model_name)
            && confidence < mv.confidence_threshold
        {
            return false;
        }

        self.confidence_gate.passes(confidence)
    }
}

impl Default for ModelGovernance {
    fn default() -> Self {
        Self::new()
    }
}
