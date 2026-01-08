#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: String,
    pub suggested_fix: Option<String>,
}
