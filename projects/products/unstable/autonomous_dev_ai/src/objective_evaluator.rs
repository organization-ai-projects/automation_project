// projects/products/unstable/autonomous_dev_ai/src/objective_evaluator.rs
use serde::{Deserialize, Serialize};

use crate::{objectif_score::ObjectiveScore, objectives::Objective};

// Multi-objective evaluator
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
