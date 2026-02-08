// projects/products/unstable/autonomous_dev_ai/src/objectives.rs

use serde::{Deserialize, Serialize};

/// Multi-objective system for agent decision making
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct Objective {
    pub name: String,
    pub weight: f64,
    pub hard: bool,
    pub threshold: Option<f64>,
}

impl Objective {
    pub fn new(name: String, weight: f64, hard: bool) -> Self {
        Self {
            name,
            weight,
            hard,
            threshold: None,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = Some(threshold);
        self
    }
}

/// Evaluation score for objectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveScore {
    pub objective_name: String,
    pub score: f64,
    pub passed: bool,
    pub hard_constraint: bool,
}

/// Multi-objective evaluator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveEvaluator {
    pub objectives: Vec<Objective>,
}

impl ObjectiveEvaluator {
    pub fn new(objectives: Vec<Objective>) -> Self {
        Self { objectives }
    }

    /// Evaluate all objectives and return scores
    pub fn evaluate(&self, scores: &[(String, f64)]) -> Vec<ObjectiveScore> {
        let score_map: std::collections::HashMap<_, _> = scores.iter().cloned().collect();

        self.objectives
            .iter()
            .map(|obj| {
                let score = score_map.get(&obj.name).copied().unwrap_or(0.0);
                let passed = if let Some(threshold) = obj.threshold {
                    score >= threshold
                } else {
                    true
                };

                ObjectiveScore {
                    objective_name: obj.name.clone(),
                    score,
                    passed,
                    hard_constraint: obj.hard,
                }
            })
            .collect()
    }

    /// Check if all hard objectives are satisfied
    pub fn hard_objectives_satisfied(&self, scores: &[ObjectiveScore]) -> bool {
        scores
            .iter()
            .filter(|s| s.hard_constraint)
            .all(|s| s.passed)
    }

    /// Calculate weighted total score
    pub fn weighted_score(&self, scores: &[ObjectiveScore]) -> f64 {
        let score_map: std::collections::HashMap<_, _> = scores
            .iter()
            .map(|s| (s.objective_name.as_str(), s.score))
            .collect();

        self.objectives
            .iter()
            .map(|obj| {
                let score = score_map.get(obj.name.as_str()).copied().unwrap_or(0.0);
                score * obj.weight
            })
            .sum()
    }
}

/// Default objectives as specified in the requirements
pub fn default_objectives() -> Vec<Objective> {
    vec![
        Objective::new("task_completion".to_string(), 1.0, true).with_threshold(1.0),
        Objective::new("policy_safety".to_string(), 1.0, true).with_threshold(1.0),
        Objective::new("tests_pass".to_string(), 0.9, true).with_threshold(1.0),
        Objective::new("minimal_diff".to_string(), 0.6, false),
        Objective::new("time_budget".to_string(), 0.4, false),
    ]
}
