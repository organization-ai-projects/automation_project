use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThesisSnapshot {
    pub ticker: String,
    pub date: String,
    pub thesis_statement: String,
    pub key_assumptions: Vec<String>,
    pub invalidation_triggers: Vec<String>,
}

impl ThesisSnapshot {
    pub fn new(
        ticker: impl Into<String>,
        date: impl Into<String>,
        thesis_statement: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            date: date.into(),
            thesis_statement: thesis_statement.into(),
            key_assumptions: Vec::new(),
            invalidation_triggers: Vec::new(),
        }
    }
}
