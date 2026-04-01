use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn apply(state: &mut AppState, action: &Action) {
        match action {
            Action::RunRequested => {
                state.running = true;
            }
            Action::RunCompleted => {
                state.running = false;
            }
            Action::ToggleGravity => {
                state.gravity_enabled = !state.gravity_enabled;
            }
            Action::ToggleElectromagnetism => {
                state.electromagnetism_enabled = !state.electromagnetism_enabled;
            }
            Action::ToggleStrongNuclear => {
                state.strong_nuclear_enabled = !state.strong_nuclear_enabled;
            }
            Action::ToggleWeakNuclear => {
                state.weak_nuclear_enabled = !state.weak_nuclear_enabled;
            }
            Action::ToggleDarkMatter => {
                state.dark_matter_enabled = !state.dark_matter_enabled;
            }
            Action::ToggleDarkEnergy => {
                state.dark_energy_enabled = !state.dark_energy_enabled;
            }
            Action::ToggleThermodynamics => {
                state.thermodynamics_enabled = !state.thermodynamics_enabled;
            }
            Action::SetSeed(seed) => {
                state.seed = *seed;
            }
            Action::SetTicks(ticks) => {
                state.ticks = *ticks;
            }
            Action::SetTicksPerEra(tpe) => {
                state.ticks_per_era = *tpe;
            }
        }
    }
}
