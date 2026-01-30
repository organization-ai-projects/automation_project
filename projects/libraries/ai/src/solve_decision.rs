// projects/libraries/ai/src/solve_decision.rs
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SolveDecision {
    /// No special note.
    #[default]
    None,

    /// Symbolic had high confidence; neural not needed.
    SymbolicHighConfidence,

    /// Symbolic failed or had low confidence; fallback to neural.
    SymbolicFailedOrLowConfidence,

    /// Neural output validated by symbolic.
    NeuralValidated,

    /// Neural output rejected/invalid; fallback to symbolic.
    NeuralInvalidFallbackToSymbolic,

    /// Hybrid compared both and chose the best confidence.
    HybridBestOfBoth,

    /// Only one solver was available or succeeded.
    OnlyOneSolverAvailable,
}
