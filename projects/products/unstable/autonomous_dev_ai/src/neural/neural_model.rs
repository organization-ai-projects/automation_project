// projects/products/unstable/autonomous_dev_ai/src/neural/neural_model.rs
use crate::error::AgentResult;
use crate::symbolic::NeuralProposal;

/// Neural model interface
pub trait NeuralModel {
    fn infer(&self, input: &str) -> AgentResult<NeuralProposal>;
    fn confidence(&self) -> f64;
}
