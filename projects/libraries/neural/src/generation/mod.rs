// projects/libraries/neural/src/generation/mod.rs
pub mod code_generator;
pub mod generation_config;
pub mod generation_error;
pub mod probabilities;
pub mod sampling;

pub use generation_config::GenerationConfig;
pub use generation_error::GenerationError;
pub use probabilities::softmax;
pub use sampling::{apply_top_k, sample_categorical};
