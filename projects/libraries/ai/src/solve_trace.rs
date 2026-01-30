// projects/libraries/ai/src/solve_trace.rs
use crate::solve_decision::SolveDecision;
use crate::solve_winner::SolveWinner;
use crate::solver_strategy::SolverStrategy;

/// Structured trace of how a task was solved.
/// This is the missing piece to make neurosymbolic feedback "path-aware".
#[derive(Debug, Clone)]
pub struct SolveTrace {
    pub strategy: SolverStrategy,

    /// Which solver produced the final output.
    pub winner: Option<SolveWinner>,

    /// Whether we had to fallback from one solver to another.
    pub fallback_used: bool,

    /// Whether each solver actually ran.
    pub symbolic_ran: bool,
    pub neural_ran: bool,

    /// Whether each solver failed (e.g., runtime error, unavailable).
    pub symbolic_failed: bool,
    pub neural_failed: bool,

    /// Confidence values when available.
    pub symbolic_confidence: Option<f64>,
    pub neural_confidence: Option<f64>,

    /// Whether the symbolic system validated a neural output.
    pub symbolic_validated_neural: bool,

    /// Result of validation when it ran.
    pub validation_passed: Option<bool>,

    /// Optional short reason for fallback/decision (structured beats strings).
    pub decision: SolveDecision,
}

impl SolveTrace {
    pub fn new(strategy: SolverStrategy) -> Self {
        Self {
            strategy,
            winner: None,
            fallback_used: false,
            symbolic_ran: false,
            neural_ran: false,
            symbolic_failed: false,
            neural_failed: false,
            symbolic_confidence: None,
            neural_confidence: None,
            symbolic_validated_neural: false,
            validation_passed: None,
            decision: SolveDecision::default(),
        }
    }
}
