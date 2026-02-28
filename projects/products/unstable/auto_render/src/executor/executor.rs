use super::{ExecutorError, ExecutionResult};
use crate::plan::{Plan, ActionParameters};
use crate::policy::PolicyEngine;
use crate::world::{WorldState, WorldFingerprint, WorldEntity, EntityId, Transform, LightDescriptor, LightKind};

const LIGHTING_ENTITY_ID: u64 = 999;

pub struct Executor {
    pub policy_engine: PolicyEngine,
}

impl Executor {
    pub fn new(policy_engine: PolicyEngine) -> Self {
        Self { policy_engine }
    }

    pub fn execute(
        &self,
        plan: &Plan,
        world: &mut WorldState,
    ) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();

        for action in &plan.actions {
            self.policy_engine
                .check_action(action)
                .map_err(|e| ExecutorError::CapabilityDenied {
                    action_id: action.action_id.clone(),
                    capability: format!("{e}"),
                })?;

            self.apply_action(action, world)?;
        }

        let fingerprint = WorldFingerprint::compute(world);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            plan_id: plan.metadata.plan_id.0.clone(),
            actions_applied: plan.actions.len(),
            fingerprint: fingerprint.hash,
            elapsed_ms,
        })
    }

    fn apply_action(
        &self,
        action: &crate::plan::ActionEnvelope,
        world: &mut WorldState,
    ) -> Result<(), ExecutorError> {
        match &action.parameters {
            ActionParameters::SpawnEntity { name } => {
                let id = EntityId(world.entities.len() as u64 + 1);
                world.entities.insert(id, WorldEntity {
                    id,
                    transform: Transform::default(),
                    name: name.clone(),
                });
            }
            ActionParameters::SetTransform { entity_id, position, rotation, scale } => {
                let eid = EntityId(*entity_id);
                if let Some(entity) = world.entities.get_mut(&eid) {
                    entity.transform.position = *position;
                    entity.transform.rotation = *rotation;
                    entity.transform.scale = *scale;
                }
            }
            ActionParameters::SetCameraTransform { position, rotation } => {
                world.camera.transform.position = *position;
                world.camera.transform.rotation = *rotation;
            }
            ActionParameters::SetCameraFov { fov_deg } => {
                world.camera.fov_deg = *fov_deg;
            }
            ActionParameters::SetTrackingConstraint { target_entity_id } => {
                world.camera.tracking_target = Some(*target_entity_id);
            }
            ActionParameters::SetLighting { ambient_color, light_kind, color, intensity } => {
                world.lighting.ambient_color = *ambient_color;
                let kind = match light_kind.as_str() {
                    "Point" => LightKind::Point,
                    "Spot" => LightKind::Spot,
                    _ => LightKind::Directional,
                };
                world.lighting.lights.push(LightDescriptor {
                    entity_id: EntityId(LIGHTING_ENTITY_ID),
                    kind,
                    color: *color,
                    intensity: *intensity,
                });
            }
            ActionParameters::SetComponent { .. } => {}
            ActionParameters::SetAssetSpec { .. } => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::plan::{
        ActionEnvelope, ActionType, ActionParameters, Capability,
        Plan, PlanMetadata, PlanId, PlanSchemaVersion,
    };
    use crate::policy::{PolicyEngine, PolicySnapshot, CapabilitySet, Budget, ApprovalRule};

    fn make_engine(caps: HashSet<Capability>) -> PolicyEngine {
        PolicyEngine::new(PolicySnapshot {
            snapshot_id: "snap-001".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            allowed_capabilities: CapabilitySet::new(caps),
            budget: Budget::default(),
            rules: vec![ApprovalRule::AutoApprove],
        })
    }

    fn make_plan(actions: Vec<ActionEnvelope>) -> Plan {
        Plan {
            metadata: PlanMetadata {
                plan_id: PlanId("p1".to_string()),
                plan_schema_version: PlanSchemaVersion { major: 1, minor: 0, patch: 0 },
                engine_version: "0.1.0".to_string(),
                planner_id: "test".to_string(),
                planner_version: "0.1.0".to_string(),
                policy_snapshot_id: "snap-001".to_string(),
                seed: 0,
                inputs_hash: "".to_string(),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                explain: "test".to_string(),
                explain_trace_ref: None,
            },
            actions,
        }
    }

    #[test]
    fn test_deterministic_fingerprint() {
        let mut caps = HashSet::new();
        caps.insert(Capability::WorldSpawnEntity);
        caps.insert(Capability::CameraSet);
        let executor = Executor::new(make_engine(caps.clone()));

        let actions = vec![
            ActionEnvelope {
                action_id: "a1".to_string(),
                action_type: ActionType::SpawnEntity,
                capability_required: Capability::WorldSpawnEntity,
                parameters: ActionParameters::SpawnEntity { name: "subject".to_string() },
                preconditions: vec![],
                postconditions: vec![],
            },
            ActionEnvelope {
                action_id: "a2".to_string(),
                action_type: ActionType::SetCameraFov,
                capability_required: Capability::CameraSet,
                parameters: ActionParameters::SetCameraFov { fov_deg: 50.0 },
                preconditions: vec![],
                postconditions: vec![],
            },
        ];

        let plan = make_plan(actions.clone());
        let mut world1 = WorldState::new();
        let result1 = executor.execute(&plan, &mut world1).expect("execute 1");

        let executor2 = Executor::new(make_engine(caps));
        let plan2 = make_plan(actions);
        let mut world2 = WorldState::new();
        let result2 = executor2.execute(&plan2, &mut world2).expect("execute 2");

        assert_eq!(result1.fingerprint, result2.fingerprint);
    }

    #[test]
    fn test_capability_denied_during_execution() {
        let caps = HashSet::new();
        let executor = Executor::new(make_engine(caps));

        let actions = vec![ActionEnvelope {
            action_id: "a1".to_string(),
            action_type: ActionType::SpawnEntity,
            capability_required: Capability::WorldSpawnEntity,
            parameters: ActionParameters::SpawnEntity { name: "subject".to_string() },
            preconditions: vec![],
            postconditions: vec![],
        }];

        let plan = make_plan(actions);
        let mut world = WorldState::new();
        let result = executor.execute(&plan, &mut world);
        assert!(matches!(result, Err(ExecutorError::CapabilityDenied { .. })));
    }
}
