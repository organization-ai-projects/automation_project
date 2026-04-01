use crate::math::vec3::Vec3;
use crate::particles::particle::Particle;

const CONFINEMENT_STRENGTH: f64 = 1e5;
const ASYMPTOTIC_FREEDOM_RANGE: f64 = 3e-15; // 3 femtometers

pub struct StrongNuclearEngine;

impl StrongNuclearEngine {
    pub fn compute_force(displacement: &Vec3) -> Vec3 {
        let r = displacement.length();
        if r < 1e-30 {
            return Vec3::zero();
        }
        let magnitude = if r < ASYMPTOTIC_FREEDOM_RANGE {
            // Asymptotic freedom: force weakens at very short range
            CONFINEMENT_STRENGTH * (r / ASYMPTOTIC_FREEDOM_RANGE)
        } else {
            // Confinement: constant (linear potential) beyond range
            CONFINEMENT_STRENGTH
        };
        displacement.normalized().scale(-magnitude)
    }

    pub fn apply_to_particles(particles: &mut [Particle], dt: f64) {
        let len = particles.len();
        if len < 2 {
            return;
        }
        let data: Vec<(usize, Vec3, bool, bool)> = particles
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.position, p.kind.interacts_strongly(), p.alive))
            .collect();

        for i in 0..len {
            if !data[i].3 || !data[i].2 {
                continue;
            }
            let mut force = Vec3::zero();
            for j in 0..len {
                if i == j || !data[j].3 || !data[j].2 {
                    continue;
                }
                let displacement = data[j].1 - data[i].1;
                if displacement.length() < ASYMPTOTIC_FREEDOM_RANGE * 10.0 {
                    force += Self::compute_force(&displacement);
                }
            }
            particles[i].apply_force(force, dt);
        }
    }
}
