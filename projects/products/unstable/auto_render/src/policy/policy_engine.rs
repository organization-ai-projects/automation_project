use super::{ApprovalRule, PolicyError, PolicySnapshot};
use crate::plan::{ActionEnvelope, Plan, PlanCandidate};

pub struct PolicyEngine {
    pub snapshot: PolicySnapshot,
}

impl PolicyEngine {
    pub fn new(snapshot: PolicySnapshot) -> Self {
        Self { snapshot }
    }

    pub fn check_action(&self, action: &ActionEnvelope) -> Result<(), PolicyError> {
        if self.snapshot.snapshot_id.trim().is_empty() {
            return Err(PolicyError::NoSnapshot);
        }

        if !self
            .snapshot
            .allowed_capabilities
            .contains(&action.capability_required)
        {
            return Err(PolicyError::CapabilityDenied {
                action_id: action.action_id.clone(),
                required: format!("{:?}", action.capability_required),
            });
        }
        Ok(())
    }

    pub fn approve_plan_candidate(&self, candidate: &PlanCandidate) -> Result<Plan, PolicyError> {
        if self.snapshot.snapshot_id.trim().is_empty() {
            return Err(PolicyError::NoSnapshot);
        }

        if let Some(ApprovalRule::RequireApproval { reason }) = self
            .snapshot
            .rules
            .iter()
            .find(|rule| matches!(rule, ApprovalRule::RequireApproval { .. }))
        {
            return Err(PolicyError::RequiresApproval {
                reason: reason.clone(),
            });
        }

        if candidate.plan.actions.len() > self.snapshot.budget.max_actions_per_plan {
            return Err(PolicyError::BudgetExceeded {
                kind: "max_actions_per_plan".to_string(),
            });
        }
        for action in &candidate.plan.actions {
            self.check_action(action)?;
        }
        let mut plan = candidate.plan.clone();
        plan.metadata.policy_snapshot_id = self.snapshot.snapshot_id.clone();
        Ok(plan)
    }
}
