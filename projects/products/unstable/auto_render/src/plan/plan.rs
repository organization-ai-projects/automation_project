use serde::{Deserialize, Serialize};
use super::{PlanMetadata, ActionEnvelope};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub metadata: PlanMetadata,
    pub actions: Vec<ActionEnvelope>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{
        PlanId, PlanSchemaVersion, ActionType, Capability,
        ActionParameters, Precondition, Postcondition,
    };

    fn make_plan() -> Plan {
        Plan {
            metadata: PlanMetadata {
                plan_id: PlanId("test-plan-001".to_string()),
                plan_schema_version: PlanSchemaVersion { major: 1, minor: 0, patch: 0 },
                engine_version: "0.1.0".to_string(),
                planner_id: "cinematography_planner".to_string(),
                planner_version: "0.1.0".to_string(),
                policy_snapshot_id: "snap-001".to_string(),
                seed: 42,
                inputs_hash: "abc123".to_string(),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                explain: "test plan".to_string(),
                explain_trace_ref: None,
            },
            actions: vec![
                ActionEnvelope {
                    action_id: "a1".to_string(),
                    action_type: ActionType::SpawnEntity,
                    capability_required: Capability::WorldSpawnEntity,
                    parameters: ActionParameters::SpawnEntity { name: "subject".to_string() },
                    preconditions: vec![Precondition { description: "world ready".to_string() }],
                    postconditions: vec![Postcondition { description: "entity spawned".to_string() }],
                },
            ],
        }
    }

    #[test]
    fn plan_roundtrip_ron() {
        let plan = make_plan();
        let serialized = ron::to_string(&plan).expect("serialize");
        let deserialized: Plan = ron::from_str(&serialized).expect("deserialize");
        assert_eq!(plan.metadata.plan_id, deserialized.metadata.plan_id);
        assert_eq!(plan.actions.len(), deserialized.actions.len());
    }
}
