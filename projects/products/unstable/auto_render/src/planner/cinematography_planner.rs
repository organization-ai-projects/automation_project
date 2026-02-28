use super::{ConstraintReport, ExplanationTrace, PlannerError, PlannerInput};
use crate::intent::IntentPayload;
use crate::plan::{
    ActionEnvelope, ActionParameters, ActionType, Capability, Plan, PlanCandidate, PlanId,
    PlanMetadata, PlanSchemaVersion, Postcondition, Precondition, RandomnessRecord,
};

pub struct CinematographyPlanner;

impl CinematographyPlanner {
    pub fn new() -> Self {
        Self
    }

    pub fn plan(&self, input: &PlannerInput) -> Result<Vec<PlanCandidate>, PlannerError> {
        let IntentPayload::Cinematography(ref payload) = input.intent.payload;

        let fov = payload
            .fov_deg
            .unwrap_or_else(|| Self::default_fov(&payload.shot_type));
        let distance = payload
            .camera_distance
            .unwrap_or_else(|| Self::default_distance(&payload.shot_type));

        let candidate1 = self.build_candidate(input, fov, distance, 0, "primary")?;
        let candidate2 = self.build_candidate(input, fov * 1.1, distance * 1.05, 1, "alternate")?;

        Ok(vec![candidate1, candidate2])
    }

    fn default_fov(shot_type: &str) -> f64 {
        match shot_type {
            "close_up" => 50.0,
            "wide" => 24.0,
            "medium" => 35.0,
            _ => 50.0,
        }
    }

    fn default_distance(shot_type: &str) -> f64 {
        match shot_type {
            "close_up" => 1.5,
            "wide" => 10.0,
            "medium" => 3.0,
            _ => 3.0,
        }
    }

    fn build_candidate(
        &self,
        input: &PlannerInput,
        fov: f64,
        distance: f64,
        idx: u64,
        label: &str,
    ) -> Result<PlanCandidate, PlannerError> {
        let IntentPayload::Cinematography(ref payload) = input.intent.payload;

        let actions = vec![
            ActionEnvelope {
                action_id: format!("{label}-spawn-subject"),
                action_type: ActionType::SpawnEntity,
                capability_required: Capability::WorldSpawnEntity,
                parameters: ActionParameters::SpawnEntity {
                    name: payload.subject_description.clone(),
                },
                preconditions: vec![Precondition {
                    description: "World is empty or accepts new entities".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Subject entity exists in world".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-transform"),
                action_type: ActionType::SetTransform,
                capability_required: Capability::WorldSetTransform,
                parameters: ActionParameters::SetTransform {
                    entity_id: 1,
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                },
                preconditions: vec![Precondition {
                    description: "Subject entity exists".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Subject positioned at origin".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-camera-transform"),
                action_type: ActionType::SetCameraTransform,
                capability_required: Capability::CameraSet,
                parameters: ActionParameters::SetCameraTransform {
                    position: [0.0, 0.0, distance],
                    rotation: [0.0, 0.0, 0.0, 1.0],
                },
                preconditions: vec![Precondition {
                    description: "Camera exists".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Camera positioned".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-camera-fov"),
                action_type: ActionType::SetCameraFov,
                capability_required: Capability::CameraSet,
                parameters: ActionParameters::SetCameraFov { fov_deg: fov },
                preconditions: vec![Precondition {
                    description: "Camera exists".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Camera FOV set".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-lighting"),
                action_type: ActionType::SetLighting,
                capability_required: Capability::LightingSet,
                parameters: Self::lighting_params(&payload.lighting_style),
                preconditions: vec![Precondition {
                    description: "Scene exists".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Lighting applied".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-asset-spec"),
                action_type: ActionType::SetAssetSpec,
                capability_required: Capability::AssetSpecify,
                parameters: ActionParameters::SetAssetSpec {
                    entity_id: 1,
                    spec: format!(
                        "subject={} background={}",
                        payload.subject_description, payload.background
                    ),
                },
                preconditions: vec![Precondition {
                    description: "Subject entity exists".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Asset specification generated for subject".to_string(),
                }],
            },
            ActionEnvelope {
                action_id: format!("{label}-set-tracking"),
                action_type: ActionType::SetTrackingConstraint,
                capability_required: Capability::CameraSet,
                parameters: ActionParameters::SetTrackingConstraint {
                    target_entity_id: 1,
                },
                preconditions: vec![Precondition {
                    description: "Subject and camera exist".to_string(),
                }],
                postconditions: vec![Postcondition {
                    description: "Camera tracks subject".to_string(),
                }],
            },
        ];

        let plan = Plan {
            metadata: PlanMetadata {
                plan_id: PlanId(format!("{}-{}", input.intent.intent_id.0, label)),
                plan_schema_version: PlanSchemaVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
                engine_version: "0.1.0".to_string(),
                planner_id: "cinematography_planner".to_string(),
                planner_version: "0.1.0".to_string(),
                policy_snapshot_id: input.policy_snapshot.snapshot_id.clone(),
                seed: idx,
                inputs_hash: format!("{:?}", &input.intent.intent_id.0),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                explain: format!(
                    "Cinematography plan for '{}' shot_type={} lighting={} fov={:.1}",
                    payload.subject_description, payload.shot_type, payload.lighting_style, fov
                ),
                explain_trace_ref: None,
            },
            actions,
        };

        let constraint_report = ConstraintReport {
            satisfied: vec![
                format!("shot_type={}", payload.shot_type),
                format!("lighting_style={}", payload.lighting_style),
                "subject_present".to_string(),
            ],
            violated: vec![],
        };

        Ok(PlanCandidate {
            score: if idx == 0 { 1.0 } else { 0.85 },
            constraints_satisfied: constraint_report.satisfied.clone(),
            constraints_violated: constraint_report.violated.clone(),
            explanation_trace: ExplanationTrace {
                summary: plan.metadata.explain.clone(),
                key_decisions: vec![
                    format!("fov={:.1}", fov),
                    format!("distance={:.2}", distance),
                ],
                constraint_report,
            },
            randomness_record: RandomnessRecord {
                seed: idx,
                transcript_ref: None,
            },
            plan,
        })
    }

    fn lighting_params(style: &str) -> ActionParameters {
        let (ambient, color, intensity) = match style {
            "soft_studio" => ([0.2f32, 0.2, 0.2], [1.0f32, 0.98, 0.95], 0.8f32),
            "natural" => ([0.3f32, 0.3, 0.25], [1.0f32, 0.95, 0.85], 1.0f32),
            "dramatic" => ([0.05f32, 0.05, 0.05], [1.0f32, 0.9, 0.7], 1.5f32),
            _ => ([0.2f32, 0.2, 0.2], [1.0f32, 1.0, 1.0], 1.0f32),
        };
        ActionParameters::SetLighting {
            ambient_color: ambient,
            light_kind: format!("{:?}", crate::world::LightKind::Directional),
            color,
            intensity,
        }
    }
}

impl Default for CinematographyPlanner {
    fn default() -> Self {
        Self::new()
    }
}
