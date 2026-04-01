use crate::math::constants::GRAVITATIONAL_CONSTANT;
use crate::math::vec3::Vec3;
use crate::particles::particle::Particle;
use crate::structures::galaxy::Galaxy;

pub struct GravityEngine;

impl GravityEngine {
    pub fn compute_force(m1: f64, m2: f64, displacement: &Vec3) -> Vec3 {
        let r_sq = displacement.length_squared();
        if r_sq < 1e-20 {
            return Vec3::zero();
        }
        let magnitude = GRAVITATIONAL_CONSTANT * m1 * m2 / r_sq;
        displacement.normalized().scale(-magnitude)
    }

    pub fn apply_to_particles(particles: &mut [Particle], dt: f64) {
        let len = particles.len();
        if len < 2 {
            return;
        }
        let positions: Vec<(usize, Vec3, f64, bool)> = particles
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.position, p.mass, p.alive))
            .collect();

        for i in 0..len {
            if !positions[i].3 || positions[i].2 < 1e-50 {
                continue;
            }
            let mut force = Vec3::zero();
            for j in 0..len {
                if i == j || !positions[j].3 || positions[j].2 < 1e-50 {
                    continue;
                }
                let displacement = positions[j].1 - positions[i].1;
                force += Self::compute_force(positions[i].2, positions[j].2, &displacement);
            }
            particles[i].apply_force(force, dt);
        }
    }

    pub fn apply_to_galaxies(galaxies: &mut [Galaxy], dt: f64) {
        let len = galaxies.len();
        if len < 2 {
            return;
        }
        let data: Vec<(Vec3, f64)> = galaxies
            .iter()
            .map(|g| (g.position, g.total_mass()))
            .collect();

        for i in 0..len {
            let mut force = Vec3::zero();
            for j in 0..len {
                if i == j {
                    continue;
                }
                let displacement = data[j].0 - data[i].0;
                force += Self::compute_force(data[i].1, data[j].1, &displacement);
            }
            if data[i].1 > 1e-50 {
                let accel = force.scale(1.0 / data[i].1);
                galaxies[i].velocity += accel.scale(dt);
                galaxies[i].position += galaxies[i].velocity.scale(dt);
            }
        }
    }
}
