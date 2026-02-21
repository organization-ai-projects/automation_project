use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    memory::{DecisionEntry, FailureEntry, ObjectiveEvaluationEntry},
    plan_entry::PlanEntry,
    value_types::ActionName,
};

/// Memory graph for agent
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct MemoryGraph {
    /// Files explored
    pub explored_files: Vec<String>,

    /// Plans generated
    pub plans: Vec<PlanEntry>,

    /// Decisions made
    pub decisions: Vec<DecisionEntry>,

    /// Failures encountered
    pub failures: Vec<FailureEntry>,

    /// Objective evaluations per iteration
    pub objective_evaluations: Vec<ObjectiveEvaluationEntry>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            explored_files: Vec::new(),
            plans: Vec::new(),
            decisions: Vec::new(),
            failures: Vec::new(),
            objective_evaluations: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_explored_file(&mut self, path: String) {
        if !self.explored_files.contains(&path) {
            self.explored_files.push(path);
        }
    }

    pub fn add_plan(&mut self, iteration: usize, description: String, steps: Vec<String>) {
        self.plans.push(PlanEntry {
            iteration,
            description,
            steps,
        });
    }

    pub fn add_decision(
        &mut self,
        iteration: usize,
        description: String,
        neural_suggestion: Option<String>,
        symbolic_decision: ActionName,
    ) {
        self.decisions.push(DecisionEntry {
            iteration,
            description,
            neural_suggestion,
            symbolic_decision,
        });
    }

    pub fn add_failure(
        &mut self,
        iteration: usize,
        description: String,
        error: String,
        recovery_action: Option<String>,
    ) {
        self.failures.push(FailureEntry {
            iteration,
            description,
            error,
            recovery_action,
        });
    }

    pub fn add_objective_evaluation(
        &mut self,
        iteration: usize,
        scores: Vec<(String, f64)>,
        passed: bool,
    ) {
        self.objective_evaluations.push(ObjectiveEvaluationEntry {
            iteration,
            scores,
            passed,
        });
    }
}

impl Default for MemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}
