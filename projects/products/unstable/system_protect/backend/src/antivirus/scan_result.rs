use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub is_clean: bool,
    pub matched_signatures: Vec<String>,
    pub severity: String,
}

impl ScanResult {
    pub fn clean() -> Self {
        Self {
            is_clean: true,
            matched_signatures: Vec::new(),
            severity: "none".to_string(),
        }
    }

    pub fn infected(signatures: Vec<String>, severity: impl Into<String>) -> Self {
        Self {
            is_clean: false,
            matched_signatures: signatures,
            severity: severity.into(),
        }
    }
}
