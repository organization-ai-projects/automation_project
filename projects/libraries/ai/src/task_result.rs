// projects/libraries/ai/src/task_result.rs
use crate::solver_strategy::SolverStrategy;

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub output: String,
    pub confidence: f64,
    pub strategy_used: SolverStrategy,
    pub metadata: Option<String>,
}

impl TaskResult {
    /// Fonction utilitaire pour construire un TaskResult
    pub fn new(
        output: String,
        confidence: f64,
        strategy: SolverStrategy,
        metadata: Option<String>,
    ) -> Self {
        TaskResult {
            output,
            confidence,
            strategy_used: strategy,
            metadata,
        }
    }
}
