use crate::solver_strategy::SolverStrategy;

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub output: String,
    pub confidence: f64,
    pub strategy_used: SolverStrategy,
    pub metadata: Option<String>,
}
