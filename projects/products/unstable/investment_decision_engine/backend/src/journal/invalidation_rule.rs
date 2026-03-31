use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidationRule {
    pub rule_id: String,
    pub description: String,
    pub condition: String,
    pub triggered: bool,
}

impl InvalidationRule {
    pub fn new(
        rule_id: impl Into<String>,
        description: impl Into<String>,
        condition: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            description: description.into(),
            condition: condition.into(),
            triggered: false,
        }
    }

    pub fn trigger(&mut self) {
        self.triggered = true;
    }
}
