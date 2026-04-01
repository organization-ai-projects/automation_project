use crate::math::constants::COULOMB_CONSTANT;
use crate::math::vec3::Vec3;
use crate::particles::particle::Particle;

pub struct ElectromagnetismEngine;

impl ElectromagnetismEngine {
    pub fn compute_force(q1: f64, q2: f64, displacement: &Vec3) -> Vec3 {
        let r_sq = displacement.length_squared();
        if r_sq < 1e-30 {
            return Vec3::zero();
        }
        let magnitude = COULOMB_CONSTANT * q1 * q2 / r_sq;
        displacement.normalized().scale(magnitude)
    }

    pub fn apply_to_particles(particles: &mut [Particle], dt: f64) {
        let len = particles.len();
        if len < 2 {
            return;
        }
        let data: Vec<(usize, Vec3, f64, f64, bool)> = particles
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.position, p.charge, p.mass, p.alive))
            .collect();

        for i in 0..len {
            if !data[i].4 || data[i].2.abs() < 1e-50 {
                continue;
            }
            let mut force = Vec3::zero();
            for j in 0..len {
                if i == j || !data[j].4 || data[j].2.abs() < 1e-50 {
                    continue;
                }
                let displacement = data[j].1 - data[i].1;
                force += Self::compute_force(data[i].2, data[j].2, &displacement);
            }
            particles[i].apply_force(force, dt);
        }
    }
}
