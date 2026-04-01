use crate::math::vec3::Vec3;
use crate::structures::structure_id::StructureId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GalaxyType {
    Spiral,
    Elliptical,
    Irregular,
    Lenticular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    pub id: StructureId,
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
    pub star_count: u64,
    pub galaxy_type: GalaxyType,
    pub dark_matter_halo_mass: f64,
    pub age_ticks: u64,
}

impl Galaxy {
    pub fn new(
        id: StructureId,
        position: Vec3,
        mass: f64,
        galaxy_type: GalaxyType,
    ) -> Self {
        Self {
            id,
            position,
            velocity: Vec3::zero(),
            mass,
            star_count: (mass / 2e30).max(1.0) as u64,
            galaxy_type,
            dark_matter_halo_mass: mass * 5.0,
            age_ticks: 0,
        }
    }

    pub fn total_mass(&self) -> f64 {
        self.mass + self.dark_matter_halo_mass
    }

    pub fn evolve(&mut self) {
        self.age_ticks += 1;
        self.star_count = (self.star_count as f64 * 1.0001) as u64;
    }
}
