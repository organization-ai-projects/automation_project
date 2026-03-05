// projects/products/unstable/hospital_tycoon/ui/src/public_api.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::app::reducer::Reducer;
use crate::diagnostics::app_error::AppError;
use crate::fixtures::fixture_loader::FixtureLoader;
use crate::screens::report_screen::ReportScreen;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use std::path::PathBuf;

pub struct UiApi;

impl UiApi {
    pub fn run(scenario: PathBuf, seed: u64, ticks: u64) -> Result<(), AppError> {
        let _scenario_json = FixtureLoader::load_json(&scenario)?;
        let process = BackendProcess::spawn(&scenario)?;
        let mut client = IpcClient::new(process);
        let mut state = AppState::new(seed, ticks);
        let mut controller = Controller::new();

        controller.init(&mut client, &mut state, seed, ticks)?;
        controller.run_to_end(&mut client, &mut state)?;
        controller.print_report(&mut client, &mut state)?;
        let replay_path = std::env::temp_dir().join("hospital_tycoon_last_run.replay.json");
        let replay_path_str = replay_path.to_string_lossy().to_string();
        controller.save_replay(&mut client, &replay_path_str)?;
        Reducer::apply(&mut state, &Action::SaveReplay(replay_path_str));
        Reducer::apply(&mut state, &Action::Quit);

        client.shutdown();
        Ok(())
    }

    pub fn replay(scenario: PathBuf, replay_path: PathBuf) -> Result<(), AppError> {
        let _scenario_json = FixtureLoader::load_json(&scenario)?;
        let process = BackendProcess::spawn(&scenario)?;
        let mut client = IpcClient::new(process);
        let mut state = AppState::new(0, 0);
        let mut controller = Controller::new();

        let replay_str = replay_path
            .to_str()
            .ok_or_else(|| AppError::Replay("invalid replay path".to_string()))?;
        Reducer::apply(&mut state, &Action::LoadReplay(replay_str.to_string()));
        client.load_replay(replay_str)?;
        let report = controller.replay_to_end(&mut client, &mut state)?;
        Reducer::apply(&mut state, &Action::GetReport);

        let screen = ReportScreen::new(report);
        screen.render();
        Reducer::apply(&mut state, &Action::Quit);

        client.shutdown();
        Ok(())
    }
}
