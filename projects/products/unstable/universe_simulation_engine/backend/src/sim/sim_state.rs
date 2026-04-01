use crate::config::physics_config::PhysicsConfig;
use crate::cosmology::cosmic_parameters::CosmicParameters;
use crate::cosmology::era::Era;
use crate::particles::particle::Particle;
use crate::spatial::grid::SpatialGrid;
use crate::structures::cosmic_web::CosmicWeb;
use crate::structures::galaxy::Galaxy;
use crate::structures::star::Star;
use crate::time::tick_clock::TickClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimState {
    pub clock: TickClock,
    pub era: Era,
    pub era_progress: f64,
    pub cosmic_params: CosmicParameters,
    pub physics_config: PhysicsConfig,
    pub particles: Vec<Particle>,
    pub stars: Vec<Star>,
    pub galaxies: Vec<Galaxy>,
    pub cosmic_web: CosmicWeb,
    pub spatial_grid: SpatialGrid,
    pub next_particle_id: u64,
    pub next_structure_id: u64,
}

impl SimState {
    pub fn new(physics_config: PhysicsConfig) -> Self {
        Self {
            clock: TickClock::new(),
            era: Era::Singularity,
            era_progress: 0.0,
            cosmic_params: CosmicParameters::at_era(&Era::Singularity, 0.0),
            physics_config,
            particles: Vec::new(),
            stars: Vec::new(),
            galaxies: Vec::new(),
            cosmic_web: CosmicWeb::default(),
            spatial_grid: SpatialGrid::new(1e10),
            next_particle_id: 1,
            next_structure_id: 1,
        }
    }
}
