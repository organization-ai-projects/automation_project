//! Neural computation layer - advisory only.

pub mod governance;
mod intent_interpretation;
mod neural_layer;
mod neural_model;

pub use intent_interpretation::IntentInterpretation;
pub use neural_layer::NeuralLayer;
pub use neural_model::NeuralModel;
