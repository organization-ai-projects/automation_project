use crate::error::AgentResult;
use crate::symbolic::NeuralProposal;

/// Neural model interface
pub trait NeuralModel {
    fn infer(&self, input: &str) -> AgentResult<NeuralProposal>;
    fn confidence(&self) -> f64;
}
