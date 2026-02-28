use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionParameters {
    SetTransform {
        entity_id: u64,
        position: [f64; 3],
        rotation: [f64; 4],
        scale: [f64; 3],
    },
    SetCameraTransform {
        position: [f64; 3],
        rotation: [f64; 4],
    },
    SetCameraFov {
        fov_deg: f64,
    },
    SetTrackingConstraint {
        target_entity_id: u64,
    },
    SpawnEntity {
        name: String,
    },
    SetComponent {
        entity_id: u64,
        component_name: String,
        value: String,
    },
    SetLighting {
        ambient_color: [f32; 3],
        light_kind: String,
        color: [f32; 3],
        intensity: f32,
    },
    SetAssetSpec {
        entity_id: u64,
        spec: String,
    },
    GenerateAsset {
        entity_id: u64,
    },
}
