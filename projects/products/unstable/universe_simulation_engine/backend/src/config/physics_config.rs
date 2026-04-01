use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub gravity_enabled: bool,
    pub electromagnetism_enabled: bool,
    pub strong_nuclear_enabled: bool,
    pub weak_nuclear_enabled: bool,
    pub dark_matter_enabled: bool,
    pub dark_energy_enabled: bool,
    pub thermodynamics_enabled: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_enabled: true,
            electromagnetism_enabled: true,
            strong_nuclear_enabled: true,
            weak_nuclear_enabled: true,
            dark_matter_enabled: true,
            dark_energy_enabled: true,
            thermodynamics_enabled: true,
        }
    }
}
