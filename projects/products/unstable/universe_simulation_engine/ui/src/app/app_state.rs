#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub running: bool,
    pub seed: u64,
    pub ticks: u64,
    pub ticks_per_era: u64,
    pub gravity_enabled: bool,
    pub electromagnetism_enabled: bool,
    pub strong_nuclear_enabled: bool,
    pub weak_nuclear_enabled: bool,
    pub dark_matter_enabled: bool,
    pub dark_energy_enabled: bool,
    pub thermodynamics_enabled: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: false,
            seed: 42,
            ticks: 1000,
            ticks_per_era: 50,
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
