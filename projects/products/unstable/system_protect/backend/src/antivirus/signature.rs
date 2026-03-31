use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub pattern: String,
    pub severity: String,
}

impl Signature {
    pub fn new(
        name: impl Into<String>,
        pattern: impl Into<String>,
        severity: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
            severity: severity.into(),
        }
    }

    pub fn matches(&self, payload: &str) -> bool {
        payload.contains(&self.pattern)
    }
}
