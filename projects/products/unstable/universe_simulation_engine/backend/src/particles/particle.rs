use crate::math::vec3::Vec3;
use crate::particles::particle_id::ParticleId;
use crate::particles::particle_kind::ParticleKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub id: ParticleId,
    pub kind: ParticleKind,
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
    pub charge: f64,
    pub energy: f64,
    pub alive: bool,
}

impl Particle {
    pub fn new(id: ParticleId, kind: ParticleKind, position: Vec3) -> Self {
        Self {
            id,
            kind,
            position,
            velocity: Vec3::zero(),
            mass: kind.mass_kg(),
            charge: kind.charge(),
            energy: kind.mass_kg() * (crate::math::constants::SPEED_OF_LIGHT.powi(2)),
            alive: true,
        }
    }

    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.length_squared()
    }

    pub fn apply_force(&mut self, force: Vec3, dt: f64) {
        if self.mass < 1e-50 {
            return;
        }
        let accel = force.scale(1.0 / self.mass);
        self.velocity += accel.scale(dt);
    }

    pub fn step(&mut self, dt: f64) {
        if !self.alive {
            return;
        }
        self.position += self.velocity.scale(dt);
    }
}
