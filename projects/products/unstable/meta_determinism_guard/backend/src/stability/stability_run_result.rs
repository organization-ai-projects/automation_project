#[derive(Debug, Clone)]
pub struct StabilityRunResult {
    pub run_index: u32,
    pub stdout: String,
    pub hash: String,
}
