// projects/products/unstable/digital_pet/ui/src/app/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::diagnostics::app_error::AppError;
use crate::screens::battle_screen::BattleScreen;
use crate::screens::evolution_screen::EvolutionScreen;
use crate::screens::pet_screen::PetScreen;
use crate::screens::report_screen::ReportScreen;
use crate::screens::training_screen::TrainingScreen;
use crate::transport::battle_state_dto::BattleStateDto;
use crate::transport::ipc_client::IpcClient;
use crate::transport::pet_state_dto::PetStateDto;
use crate::transport::run_report_dto::RunReportDto;
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
        scenario: &Path,
        seed: u64,
        ticks: u64,
    ) -> Result<(), AppError> {
        client.load_scenario(scenario.display().to_string())?;
        client.new_run(seed, ticks)?;
        state.running = true;
        Ok(())
    }

    pub fn run_loop(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
        replay_out: Option<&Path>,
    ) -> Result<(), AppError> {
        let step_size = 10u64;
        let mut battle_started = false;
        let mut previous_species = state.species.clone();

        while state.current_tick < state.ticks {
            let remaining = state.ticks - state.current_tick;
            let n = remaining.min(step_size);
            let pet_state = client.step(n)?;
            Reducer::apply(state, &Action::Step(n));
            self.apply_pet_state(state, pet_state.clone());
            let screen = PetScreen::new(state.clone());
            screen.render();

            if !previous_species.is_empty() && state.species != previous_species {
                EvolutionScreen {
                    from: previous_species.clone(),
                    to: state.species.clone(),
                    stage: state.evolution_stage,
                }
                .render();
            }
            previous_species = state.species.clone();

            if state.current_tick.is_multiple_of(30) {
                client.care_action("feed".to_string())?;
                Reducer::apply(state, &Action::Feed);
            }
            if state.current_tick.is_multiple_of(45) {
                client.care_action("rest".to_string())?;
                Reducer::apply(state, &Action::Rest);
            }
            if state.current_tick.is_multiple_of(50) {
                client.care_action("play".to_string())?;
                Reducer::apply(state, &Action::Play);
            }
            if state.current_tick.is_multiple_of(60) {
                client.care_action("discipline".to_string())?;
                Reducer::apply(state, &Action::Discipline);
            }
            if state.current_tick.is_multiple_of(90) {
                client.care_action("medicine".to_string())?;
                Reducer::apply(state, &Action::Medicine);
            }
            if state.current_tick.is_multiple_of(40) {
                let training_result = client.training("strength".to_string())?;
                Reducer::apply(state, &Action::Train("strength".to_string()));
                TrainingScreen::new(training_result).render();
            }
            if state.current_tick.is_multiple_of(25) {
                let _snapshot = client.get_snapshot()?;
                Reducer::apply(state, &Action::GetSnapshot);
            }
            if !battle_started && state.current_tick >= state.ticks / 2 {
                client.start_battle()?;
                Reducer::apply(state, &Action::StartBattle);
                battle_started = true;
            }
            if battle_started {
                let battle_state = client.battle_step()?;
                Reducer::apply(state, &Action::BattleStep);
                self.render_battle(battle_state.clone());
                if battle_state.finished {
                    battle_started = false;
                }
            }
        }

        if let Some(path) = replay_out {
            client.save_replay(path.display().to_string())?;
            Reducer::apply(state, &Action::SaveReplay(path.display().to_string()));
        }
        Reducer::apply(state, &Action::Quit);
        Ok(())
    }

    pub fn save_report(
        &mut self,
        client: &mut IpcClient,
        state: &mut AppState,
        path: &Path,
    ) -> Result<(), AppError> {
        Reducer::apply(state, &Action::GetReport);
        let report = client.get_report()?;
        self.write_report_file(&report, path)
    }

    pub fn run_replay(
        &mut self,
        client: &mut IpcClient,
        replay_path: &Path,
        out_path: &Path,
    ) -> Result<(), AppError> {
        client.load_replay(replay_path.display().to_string())?;
        let mut replay_state = AppState::new(0, 0);
        Reducer::apply(
            &mut replay_state,
            &Action::LoadReplay(replay_path.display().to_string()),
        );
        let report = client.replay_to_end()?;
        Reducer::apply(&mut replay_state, &Action::ReplayToEnd);
        self.write_report_file(&report, out_path)
    }

    fn write_report_file(&self, report: &RunReportDto, path: &Path) -> Result<(), AppError> {
        ReportScreen::new(report.clone()).render();
        let report_state = AppState::from_report(report);
        PetScreen::new(report_state).render();
        let json =
            common_json::to_string_pretty(report).map_err(|e| AppError::Ipc(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| AppError::Io(e.to_string()))
    }

    fn apply_pet_state(&self, state: &mut AppState, pet_state: PetStateDto) {
        state.current_tick = pet_state.tick;
        state.species = pet_state.species;
        state.evolution_stage = pet_state.evolution_stage;
        state.hp = pet_state.hp;
        state.max_hp = pet_state.max_hp;
        state.hunger = pet_state.hunger;
        state.fatigue = pet_state.fatigue;
        state.happiness = pet_state.happiness;
        state.discipline = pet_state.discipline;
        state.sick = pet_state.sick;
    }

    fn render_battle(&self, state: BattleStateDto) {
        BattleScreen {
            turn: state.turn,
            pet_hp: state.pet_hp,
            opponent_hp: state.opponent_hp,
            finished: state.finished,
            winner: state.winner,
        }
        .render();
    }
}
