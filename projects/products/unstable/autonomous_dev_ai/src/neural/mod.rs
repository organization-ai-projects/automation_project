//! Neural computation layer - advisory only.

mod confidence_gate;
mod drift_detector;
mod intent_interpretation;
mod model_governance;
mod model_registry;
mod model_version;
mod neural_layer;
mod neural_model;
mod rollout_state;

pub use confidence_gate::ConfidenceGate;
pub use drift_detector::DriftDetector;
pub use intent_interpretation::IntentInterpretation;
pub use model_governance::ModelGovernance;
pub use model_registry::ModelRegistry;
pub use model_version::ModelVersion;
pub use neural_layer::NeuralLayer;
pub use neural_model::NeuralModel;
pub use rollout_state::RolloutState;
