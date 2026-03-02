// projects/products/unstable/digital_pet/ui/src/app/controller.rs
use crate::app::app_state::AppState;
use crate::diagnostics::error::AppError;
use crate::screens::pet_screen::PetScreen;
use crate::screens::report_screen::ReportScreen;
use crate::transport::ipc_client::IpcClient;
use std::path::Path;

pub struct Controller;

impl Controller {
    pub fn new() -> Self {
        Self
    }

    pub fn init(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
        seed: u64,
        ticks: u64,
    ) -> Result<(), AppError> {
        client.new_run(seed, ticks)?;
        state.running = true;
        Ok(())
    }

    pub fn run_loop(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
    ) -> Result<(), AppError> {
        let step_size = 10u64;
        while state.current_tick < state.ticks {
            let remaining = state.ticks - state.current_tick;
            let n = remaining.min(step_size);
            let pet_state = client.step(n)?;
            state.current_tick += n;
            state.species = pet_state.species;
            state.evolution_stage = pet_state.evolution_stage;
            state.hp = pet_state.hp;
            state.max_hp = pet_state.max_hp;
            state.hunger = pet_state.hunger;
            state.fatigue = pet_state.fatigue;
            state.happiness = pet_state.happiness;
            state.discipline = pet_state.discipline;
            state.sick = pet_state.sick;

            let screen = PetScreen::new(state.clone());
            screen.render();
        }
        Ok(())
    }

    pub fn save_report(&mut self, client: &mut IpcClient, path: &Path) -> Result<(), AppError> {
        let report = client.get_report()?;
        let screen = ReportScreen::new(report.clone());
        screen.render();
        let json =
            serde_json::to_string_pretty(&report).map_err(|e| AppError::Ipc(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| AppError::Io(e.to_string()))
    }
}
