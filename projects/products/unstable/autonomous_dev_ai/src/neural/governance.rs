// projects/products/unstable/autonomous_dev_ai/src/neural/governance.rs

//! Neural model governance and rollout safety.
//!
//! Provides a model registry with version pinning, rollout policies,
//! confidence calibration, drift detection, and rollback procedures.
//! Symbolic override remains authoritative under all model states.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Model Version ────────────────────────────────────────────────────────────

/// A pinned model version entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub name: String,
    pub version: String,
    pub path: String,
    /// Minimum confidence score this model must produce to be trusted.
    pub confidence_threshold: f64,
    /// Whether the model is currently active in production.
    pub active: bool,
}

impl ModelVersion {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        path: impl Into<String>,
        confidence_threshold: f64,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            path: path.into(),
            confidence_threshold,
            active: false,
        }
    }
}

// ─── Rollout State ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RolloutState {
    /// Model is registered but not yet evaluated.
    Pending,
    /// Model passed offline evaluation and is being canary-deployed.
    Canary,
    /// Model is fully deployed to production.
    Production,
    /// Model has been rolled back due to drift or failure.
    RolledBack,
}

// ─── Model Registry ───────────────────────────────────────────────────────────

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

    /// Advance a model from Pending → Canary after passing offline evaluation.
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

    /// Advance a model from Canary → Production after online evaluation passes.
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

// ─── Confidence Calibration ───────────────────────────────────────────────────

/// Validates a model output's confidence and decides whether to use it or fall back.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceGate {
    /// Overall minimum confidence required to use the neural suggestion.
    pub min_confidence: f64,
}

impl ConfidenceGate {
    pub fn new(min_confidence: f64) -> Self {
        Self { min_confidence }
    }

    /// Returns true if the confidence is sufficient to trust the neural output.
    pub fn passes(&self, confidence: f64) -> bool {
        confidence >= self.min_confidence
    }
}

impl Default for ConfidenceGate {
    fn default() -> Self {
        Self::new(0.7)
    }
}

// ─── Drift Detector ───────────────────────────────────────────────────────────

/// Simple sliding-window drift detector based on rolling confidence averages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetector {
    window: Vec<f64>,
    window_size: usize,
    /// Alert when rolling average drops below this threshold.
    drift_threshold: f64,
}

impl DriftDetector {
    pub fn new(window_size: usize, drift_threshold: f64) -> Self {
        Self {
            window: Vec::with_capacity(window_size),
            window_size,
            drift_threshold,
        }
    }

    /// Record a new confidence observation; returns true if drift is detected.
    pub fn observe(&mut self, confidence: f64) -> bool {
        if self.window.len() >= self.window_size {
            self.window.remove(0);
        }
        self.window.push(confidence);

        if self.window.len() < self.window_size {
            // Not enough samples yet
            return false;
        }

        let avg = self.window.iter().sum::<f64>() / self.window.len() as f64;
        avg < self.drift_threshold
    }

    pub fn rolling_average(&self) -> Option<f64> {
        if self.window.is_empty() {
            None
        } else {
            Some(self.window.iter().sum::<f64>() / self.window.len() as f64)
        }
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new(10, 0.6)
    }
}

// ─── Model Governance Controller ─────────────────────────────────────────────

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
        if let Some(mv) = self.registry.get(model_name) {
            if confidence < mv.confidence_threshold {
                return false;
            }
        }

        self.confidence_gate.passes(confidence)
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

    #[test]
    fn test_model_registry_lifecycle() {
        let mut reg = ModelRegistry::new();
        let mv = ModelVersion::new("codegen", "v2.0", "models/codegen_v2.bin", 0.8);
        reg.register(mv);

        assert_eq!(reg.state("codegen"), Some(RolloutState::Pending));
        assert!(reg.promote_to_canary("codegen"));
        assert_eq!(reg.state("codegen"), Some(RolloutState::Canary));
        assert!(reg.promote_to_production("codegen"));
        assert_eq!(reg.state("codegen"), Some(RolloutState::Production));
    }

    #[test]
    fn test_model_rollback() {
        let mut reg = ModelRegistry::new();
        let mv = ModelVersion::new("intent", "v1.0", "models/intent_v1.bin", 0.7);
        reg.register(mv);
        reg.promote_to_canary("intent");
        reg.rollback("intent", "test rollback");
        assert_eq!(reg.state("intent"), Some(RolloutState::RolledBack));
        assert!(!reg.get("intent").unwrap().active);
    }

    #[test]
    fn test_confidence_gate() {
        let gate = ConfidenceGate::new(0.75);
        assert!(gate.passes(0.9));
        assert!(gate.passes(0.75));
        assert!(!gate.passes(0.74));
    }

    #[test]
    fn test_drift_detector_triggers() {
        let mut detector = DriftDetector::new(3, 0.7);
        // Below threshold — should trigger after window fills
        assert!(!detector.observe(0.5));
        assert!(!detector.observe(0.4));
        // Third observation fills window; avg = (0.5+0.4+0.3)/3 = 0.4 < 0.7 → drift
        assert!(detector.observe(0.3));
    }

    #[test]
    fn test_drift_detector_no_drift() {
        let mut detector = DriftDetector::new(3, 0.7);
        assert!(!detector.observe(0.9));
        assert!(!detector.observe(0.85));
        assert!(!detector.observe(0.8));
    }

    #[test]
    fn test_governance_symbolic_override_on_drift() {
        // Build a governance instance with a small, known window size so the
        // test is not coupled to the `DriftDetector::default()` configuration.
        let mut gov = ModelGovernance {
            registry: ModelRegistry::new(),
            confidence_gate: ConfidenceGate::new(0.7),
            drift_detector: DriftDetector::new(3, 0.7),
        };
        let mv = ModelVersion::new("codegen", "v1", "path", 0.5);
        gov.registry.register(mv);
        gov.registry.promote_to_canary("codegen");

        // Fill window with low-confidence values (avg 0.3 < threshold 0.7).
        gov.accept("codegen", 0.3);
        gov.accept("codegen", 0.3);
        // Third observation fills the 3-slot window and triggers drift → rollback.
        let result = gov.accept("codegen", 0.3);
        assert!(
            !result,
            "symbolic override must be used when drift is detected"
        );
        assert_eq!(
            gov.registry.state("codegen"),
            Some(RolloutState::RolledBack)
        );
    }
}
