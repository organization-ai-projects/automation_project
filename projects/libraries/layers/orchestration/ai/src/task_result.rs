// projects/libraries/layers/orchestration/ai/src/task_result.rs
use crate::solve_trace::SolveTrace;
use crate::solver_strategy::SolverStrategy;

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub output: String,
    pub confidence: f64,
    pub strategy_used: SolverStrategy,
    pub metadata: Option<String>,

    /// Structured trace of how the result was obtained.
    pub(crate) trace: SolveTrace,
}

impl TaskResult {
    pub fn new(
        output: String,
        confidence: f64,
        strategy: SolverStrategy,
        metadata: Option<String>,
        trace: SolveTrace,
    ) -> Self {
        TaskResult {
            output,
            confidence,
            strategy_used: strategy,
            metadata,
            trace,
        }
    }

    pub(crate) fn trace(&self) -> &SolveTrace {
        &self.trace
    }
}
