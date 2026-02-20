/// projects/libraries/layers/orchestration/ai/src/solver_strategy.rs
/// Neuro-symbolic orchestration strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolverStrategy {
    /// Use only the symbolic solver
    SymbolicOnly,

    /// Use only the neural solver
    NeuralOnly,

    /// Try symbolic first, if it fails or confidence is low → neural
    SymbolicThenNeural,

    /// Neural generates, symbolic validates and corrects if necessary
    NeuralWithSymbolicValidation,

    /// Execute both in parallel, choose the best result
    Hybrid,
}
