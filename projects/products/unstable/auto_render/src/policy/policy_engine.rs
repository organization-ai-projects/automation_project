use super::{PolicyError, PolicySnapshot};
use crate::plan::{ActionEnvelope, Plan, PlanCandidate};

pub struct PolicyEngine {
    pub snapshot: PolicySnapshot,
}

impl PolicyEngine {
    pub fn new(snapshot: PolicySnapshot) -> Self {
        Self { snapshot }
    }

    pub fn check_action(&self, action: &ActionEnvelope) -> Result<(), PolicyError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{
        ActionEnvelope, ActionParameters, ActionType, Capability, Plan, PlanCandidate, PlanId,
        PlanMetadata, PlanSchemaVersion, RandomnessRecord,
    };
    use crate::planner::{ConstraintReport, ExplanationTrace};
    use crate::policy::{ApprovalRule, Budget, CapabilitySet};
    use std::collections::HashSet;

    fn make_snapshot(caps: HashSet<Capability>) -> PolicySnapshot {
        PolicySnapshot {
            snapshot_id: "snap-001".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            allowed_capabilities: CapabilitySet::new(caps),
            budget: Budget {
                max_actions_per_plan: 5,
                max_planner_iterations: 10,
                max_time_budget_ms: 5000,
            },
            rules: vec![ApprovalRule::AutoApprove],
        }
    }

    fn make_action(id: &str, cap: Capability) -> ActionEnvelope {
        ActionEnvelope {
            action_id: id.to_string(),
            action_type: ActionType::SpawnEntity,
            capability_required: cap,
            parameters: ActionParameters::SpawnEntity {
                name: "test".to_string(),
            },
            preconditions: vec![],
            postconditions: vec![],
        }
    }

    fn make_candidate(actions: Vec<ActionEnvelope>) -> PlanCandidate {
        PlanCandidate {
            plan: Plan {
                metadata: PlanMetadata {
                    plan_id: PlanId("p1".to_string()),
                    plan_schema_version: PlanSchemaVersion {
                        major: 1,
                        minor: 0,
                        patch: 0,
                    },
                    engine_version: "0.1.0".to_string(),
                    planner_id: "test".to_string(),
                    planner_version: "0.1.0".to_string(),
                    policy_snapshot_id: "".to_string(),
                    seed: 0,
                    inputs_hash: "".to_string(),
                    created_at: "2026-01-01T00:00:00Z".to_string(),
                    explain: "test".to_string(),
                    explain_trace_ref: None,
                },
                actions,
            },
            score: 1.0,
            constraints_satisfied: vec![],
            constraints_violated: vec![],
            explanation_trace: ExplanationTrace {
                summary: "".to_string(),
                key_decisions: vec![],
                constraint_report: ConstraintReport { satisfied: vec![], violated: vec![] },
            },
            randomness_record: RandomnessRecord {
                seed: 0,
                transcript_ref: None,
            },
        }
    }

    #[test]
    fn test_capability_allowed() {
        let mut caps = HashSet::new();
        caps.insert(Capability::WorldSpawnEntity);
        let engine = PolicyEngine::new(make_snapshot(caps));
        let action = make_action("a1", Capability::WorldSpawnEntity);
        assert!(engine.check_action(&action).is_ok());
    }

    #[test]
    fn test_capability_denied() {
        let caps = HashSet::new();
        let engine = PolicyEngine::new(make_snapshot(caps));
        let action = make_action("a1", Capability::WorldSpawnEntity);
        let result = engine.check_action(&action);
        assert!(matches!(result, Err(PolicyError::CapabilityDenied { .. })));
    }

    #[test]
    fn test_budget_exceeded() {
        let mut caps = HashSet::new();
        caps.insert(Capability::WorldSpawnEntity);
        let mut snapshot = make_snapshot(caps);
        snapshot.budget.max_actions_per_plan = 1;
        let engine = PolicyEngine::new(snapshot);
        let actions = vec![
            make_action("a1", Capability::WorldSpawnEntity),
            make_action("a2", Capability::WorldSpawnEntity),
        ];
        let candidate = make_candidate(actions);
        let result = engine.approve_plan_candidate(&candidate);
        assert!(matches!(result, Err(PolicyError::BudgetExceeded { .. })));
    }

    #[test]
    fn test_approve_attaches_snapshot_id() {
        let mut caps = HashSet::new();
        caps.insert(Capability::WorldSpawnEntity);
        let engine = PolicyEngine::new(make_snapshot(caps));
        let candidate = make_candidate(vec![make_action("a1", Capability::WorldSpawnEntity)]);
        let plan = engine.approve_plan_candidate(&candidate).expect("approve");
        assert_eq!(plan.metadata.policy_snapshot_id, "snap-001");
    }
}
