use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    SetTransform,
    SetCameraTransform,
    SetCameraFov,
    SetTrackingConstraint,
    SpawnEntity,
    SetComponent,
    SetLighting,
    SetAssetSpec,
    GenerateAsset,
}
