use super::{ExecutionResult, ExecutorError};
use crate::assets::{AssetGenerator, FileAssetGenerator};
use crate::plan::{ActionParameters, Plan};
use crate::policy::PolicyEngine;
use crate::renderer::{FrameDumpRenderer, RendererBackend};
use crate::world::{
    EntityId, LightDescriptor, LightKind, Transform, WorldEntity, WorldFingerprint, WorldState,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

const LIGHTING_ENTITY_ID: u64 = 999;

pub struct Executor {
    pub policy_engine: PolicyEngine,
    asset_generator: Box<dyn AssetGenerator + Send + Sync>,
    renderer: Box<dyn RendererBackend + Send + Sync>,
}

impl Executor {
    pub fn new(policy_engine: PolicyEngine) -> Self {
        let output_root = PathBuf::from("auto_render_output");
        let asset_generator = Box::new(FileAssetGenerator::new(output_root.join("assets")));
        let renderer = Box::new(FrameDumpRenderer::new(output_root.join("frames")));
        Self::with_backends(policy_engine, asset_generator, renderer)
    }

    pub fn with_backends(
        policy_engine: PolicyEngine,
        asset_generator: Box<dyn AssetGenerator + Send + Sync>,
        renderer: Box<dyn RendererBackend + Send + Sync>,
    ) -> Self {
        Self {
            policy_engine,
            asset_generator,
            renderer,
        }
    }

    pub fn execute(
        &self,
        plan: &Plan,
        world: &mut WorldState,
    ) -> Result<ExecutionResult, ExecutorError> {
        let start = std::time::Instant::now();
        self.validate_world(world)?;

        for action in &plan.actions {
            self.policy_engine.check_action(action).map_err(|e| {
                ExecutorError::CapabilityDenied {
                    action_id: action.action_id.clone(),
                    capability: format!("{e}"),
                }
            })?;

            self.apply_action(action, world)?;
            self.validate_world(world)?;
        }

        self.renderer
            .render_frame(world)
            .map_err(|e| ExecutorError::ActionFailed {
                action_id: "render_frame".to_string(),
                reason: e.to_string(),
            })?;

        let fingerprint = WorldFingerprint::compute(world);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            plan_id: plan.metadata.plan_id.0.clone(),
            actions_applied: plan.actions.len(),
            fingerprint: fingerprint.hash,
            elapsed_ms,
        })
    }

    fn validate_world(&self, world: &WorldState) -> Result<(), ExecutorError> {
        for (id, entity) in &world.entities {
            if *id != entity.id {
                return Err(ExecutorError::WorldCorrupted);
            }
        }

        if let Some(target) = world.camera.tracking_target
            && !world.entities.contains_key(&EntityId(target))
        {
            return Err(ExecutorError::WorldCorrupted);
        }

        Ok(())
    }

    fn apply_action(
        &self,
        action: &crate::plan::ActionEnvelope,
        world: &mut WorldState,
    ) -> Result<(), ExecutorError> {
        match &action.parameters {
            ActionParameters::SpawnEntity { name } => {
                let id = EntityId(world.entities.len() as u64 + 1);
                world.entities.insert(
                    id,
                    WorldEntity {
                        id,
                        transform: Transform::default(),
                        name: name.clone(),
                        components: BTreeMap::new(),
                    },
                );
            }
            ActionParameters::SetTransform {
                entity_id,
                position,
                rotation,
                scale,
            } => {
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
            ActionParameters::SetLighting {
                ambient_color,
                light_kind,
                color,
                intensity,
            } => {
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
            ActionParameters::SetComponent {
                entity_id,
                component_name,
                value,
            } => {
                let eid = EntityId(*entity_id);
                let Some(entity) = world.entities.get_mut(&eid) else {
                    return Err(ExecutorError::PreconditionFailed {
                        action_id: action.action_id.clone(),
                        condition: format!("entity {entity_id} does not exist"),
                    });
                };
                entity
                    .components
                    .insert(component_name.clone(), value.clone());
            }
            ActionParameters::SetAssetSpec { entity_id, spec } => {
                let eid = EntityId(*entity_id);
                let Some(entity) = world.entities.get_mut(&eid) else {
                    return Err(ExecutorError::PreconditionFailed {
                        action_id: action.action_id.clone(),
                        condition: format!("entity {entity_id} does not exist"),
                    });
                };
                entity
                    .components
                    .insert("asset_spec".to_string(), spec.clone());
            }
            ActionParameters::GenerateAsset { entity_id } => {
                let eid = EntityId(*entity_id);
                let Some(entity) = world.entities.get(&eid) else {
                    return Err(ExecutorError::PreconditionFailed {
                        action_id: action.action_id.clone(),
                        condition: format!("entity {entity_id} does not exist"),
                    });
                };
                let Some(spec) = entity.components.get("asset_spec") else {
                    return Err(ExecutorError::PreconditionFailed {
                        action_id: action.action_id.clone(),
                        condition: format!("entity {entity_id} has no asset_spec"),
                    });
                };

                self.asset_generator
                    .generate(spec)
                    .map_err(|e| ExecutorError::ActionFailed {
                        action_id: action.action_id.clone(),
                        reason: e.to_string(),
                    })?;
            }
        }
        Ok(())
    }
}
