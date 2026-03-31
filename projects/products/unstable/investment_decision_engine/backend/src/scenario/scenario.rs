use serde::{Deserialize, Serialize};

use crate::scenario::StressCase;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub stress_cases: Vec<StressCase>,
}

impl Scenario {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            stress_cases: Vec::new(),
        }
    }

    pub fn add_stress_case(&mut self, case: StressCase) {
        self.stress_cases.push(case);
    }
}
