#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub forbidden_patterns: Vec<String>,
    pub skip_dirs: Vec<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            forbidden_patterns: vec![
                "TODO".to_string(),
                "FIXME".to_string(),
                "HACK".to_string(),
                "unsafe".to_string(),
            ],
            skip_dirs: vec![".git".to_string(), "target".to_string()],
        }
    }
}
