use crate::cosmology::cosmic_parameters::CosmicParameters;
use crate::math::constants::DARK_ENERGY_DENSITY;

pub struct DarkEnergyEngine;

impl DarkEnergyEngine {
    pub fn expansion_acceleration(cosmic_params: &CosmicParameters) -> f64 {
        DARK_ENERGY_DENSITY * cosmic_params.scale_factor * 1e-10
    }

    pub fn apply_expansion(cosmic_params: &CosmicParameters) -> CosmicParameters {
        let accel = Self::expansion_acceleration(cosmic_params);
        CosmicParameters {
            temperature: cosmic_params.temperature,
            density: cosmic_params.density * (1.0 - accel * 1e-5).max(0.0),
            scale_factor: cosmic_params.scale_factor * (1.0 + accel),
            expansion_rate: cosmic_params.expansion_rate * (1.0 + accel * 0.01),
        }
    }
}
