//! Neural model governance and rollout safety.

mod confidence_gate;
mod drift_detector;
mod model_governance;
mod model_registry;
mod model_version;
mod rollout_state;

pub use confidence_gate::ConfidenceGate;
pub use drift_detector::DriftDetector;
pub use model_governance::ModelGovernance;
pub use model_registry::ModelRegistry;
pub use model_version::ModelVersion;
pub use rollout_state::RolloutState;

#[cfg(test)]
mod tests;
