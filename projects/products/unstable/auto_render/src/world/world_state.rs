use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::{EntityId, WorldEntity, CameraState, LightingState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub entities: BTreeMap<EntityId, WorldEntity>,
    pub camera: CameraState,
    pub lighting: LightingState,
    pub tick_id: u64,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::new(),
            camera: CameraState::default(),
            lighting: LightingState::default(),
            tick_id: 0,
        }
    }

    pub fn query_entities(&self) -> Vec<&WorldEntity> {
        self.entities.values().collect()
    }

    pub fn get_camera(&self) -> &CameraState {
        &self.camera
    }

    pub fn get_lighting(&self) -> &LightingState {
        &self.lighting
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
