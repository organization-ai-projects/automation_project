use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestReport {
    pub test_type: String,
    pub passed: bool,
    pub states_visited: usize,
    pub transitions_fired: usize,
    pub violations: Vec<String>,
    pub seed: Option<u64>,
    pub steps: u64,
}

impl TestReport {
    pub fn exhaustive(
        states_visited: usize,
        transitions_fired: usize,
        violations: Vec<String>,
    ) -> Self {
        Self {
            test_type: "exhaustive".to_string(),
            passed: violations.is_empty(),
            states_visited,
            transitions_fired,
            violations,
            seed: None,
            steps: 0,
        }
    }

    pub fn fuzz(seed: u64, steps: u64, violations: Vec<String>) -> Self {
        Self {
            test_type: "fuzz".to_string(),
            passed: violations.is_empty(),
            states_visited: 0,
            transitions_fired: steps as usize,
            violations,
            seed: Some(seed),
            steps,
        }
    }
}
