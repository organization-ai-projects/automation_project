use crate::cosmology::cosmic_parameters::CosmicParameters;
use crate::math::constants::BOLTZMANN_CONSTANT;
use crate::math::vec3::Vec3;
use crate::particles::particle::Particle;
use crate::rng::seeded_rng::SeededRng;

pub struct ThermodynamicsEngine;

impl ThermodynamicsEngine {
    pub fn thermal_velocity(mass: f64, temperature: f64) -> f64 {
        if mass < 1e-50 || temperature < 1e-50 {
            return 0.0;
        }
        (3.0 * BOLTZMANN_CONSTANT * temperature / mass).sqrt()
    }

    pub fn apply_thermal_motion(
        particles: &mut [Particle],
        cosmic_params: &CosmicParameters,
        rng: &mut SeededRng,
    ) {
        for p in particles.iter_mut() {
            if !p.alive || p.mass < 1e-50 {
                continue;
            }
            let v_thermal = Self::thermal_velocity(p.mass, cosmic_params.temperature);
            let dx = (rng.next_f64() - 0.5) * 2.0;
            let dy = (rng.next_f64() - 0.5) * 2.0;
            let dz = (rng.next_f64() - 0.5) * 2.0;
            let random_dir = Vec3::new(dx, dy, dz).normalized();
            let thermal_kick = random_dir.scale(v_thermal * 0.01);
            p.velocity += thermal_kick;
        }
    }

    pub fn cool_universe(
        cosmic_params: &CosmicParameters,
        expansion_factor: f64,
    ) -> CosmicParameters {
        let factor = if expansion_factor > 1e-30 {
            1.0 / expansion_factor
        } else {
            1.0
        };
        CosmicParameters {
            temperature: cosmic_params.temperature * factor.min(1.0),
            density: cosmic_params.density * factor.powf(3.0).min(1.0),
            scale_factor: cosmic_params.scale_factor * expansion_factor.max(1.0),
            expansion_rate: cosmic_params.expansion_rate,
        }
    }
}
