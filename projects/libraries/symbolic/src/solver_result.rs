#[derive(Debug, Clone)]
pub struct SolverResult {
    pub output: String,
    pub confidence: f64,
    pub metadata: Option<String>,
}
