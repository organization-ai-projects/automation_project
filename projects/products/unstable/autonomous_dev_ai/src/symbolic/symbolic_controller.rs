// projects/products/unstable/autonomous_dev_ai/src/symbolic/symbolic_controller.rs
use super::policy_engine::{FORCE_PUSH_FORBIDDEN, is_force_push_action};
use super::{NeuralProposal, ValidationResult};
use crate::error::AgentResult;
use crate::objective_evaluator::ObjectiveEvaluator;
use crate::symbolic::{CategoryDecision, IssueClassificationInput, classify_issue};

/// Symbolic controller - makes all final decisions.
#[derive(Debug)]
pub struct SymbolicController {
    pub evaluator: ObjectiveEvaluator,
    pub strict_validation: bool,
    pub deterministic: bool,
}

impl SymbolicController {
    pub fn new(
        evaluator: ObjectiveEvaluator,
        strict_validation: bool,
        deterministic: bool,
    ) -> Self {
        Self {
            evaluator,
            strict_validation,
            deterministic,
        }
    }

    /// Validate a neural proposal against symbolic rules.
    pub fn validate_proposal(&self, proposal: &NeuralProposal) -> AgentResult<ValidationResult> {
        let mut issues = Vec::new();

        if is_force_push_action(&proposal.action) {
            issues.push(format!("{FORCE_PUSH_FORBIDDEN} is not allowed"));
        }

        if proposal.action.contains("rm -rf") {
            issues.push("destructive operations require explicit approval".to_string());
        }

        let is_valid = issues.is_empty() || !self.strict_validation;

        Ok(ValidationResult {
            is_valid,
            issues,
            approved_action: if is_valid {
                Some(proposal.action.clone())
            } else {
                None
            },
        })
    }

    /// Deterministic-first category decision with latent fallback.
    pub fn resolve_issue_category(
        &self,
        input: &IssueClassificationInput,
        latent_threshold: f64,
    ) -> CategoryDecision {
        classify_issue(input, latent_threshold)
    }
}
