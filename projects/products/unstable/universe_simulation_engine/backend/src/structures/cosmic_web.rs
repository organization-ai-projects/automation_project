use crate::math::vec3::Vec3;
use crate::structures::structure_id::StructureId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filament {
    pub id: StructureId,
    pub start: Vec3,
    pub end: Vec3,
    pub mass: f64,
    pub galaxy_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Void {
    pub id: StructureId,
    pub center: Vec3,
    pub radius: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CosmicWeb {
    pub filaments: Vec<Filament>,
    pub voids: Vec<Void>,
}

impl CosmicWeb {
    pub fn add_filament(&mut self, filament: Filament) {
        self.filaments.push(filament);
    }

    pub fn add_void(&mut self, void_region: Void) {
        self.voids.push(void_region);
    }
}
