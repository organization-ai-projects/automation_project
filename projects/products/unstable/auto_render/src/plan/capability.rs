use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    WorldRead,
    WorldSpawnEntity,
    WorldSetTransform,
    WorldSetComponent,
    CameraSet,
    LightingSet,
    AssetSpecify,
    AssetGenerate,
    IoReadDisk,
    IoWriteDisk,
}
