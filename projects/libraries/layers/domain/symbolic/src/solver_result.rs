// projects/libraries/layers/domain/symbolic/src/solver_result.rs
#[derive(Debug, Clone)]
pub struct SolverResult {
    pub output: String,
    pub confidence: f64,
    pub metadata: Option<String>,
}
