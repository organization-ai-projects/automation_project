// projects/products/unstable/hospital_tycoon/ui/src/public_api.rs
use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::diagnostics::error::AppError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use std::path::PathBuf;

pub struct UiApi;

impl UiApi {
    pub fn run(scenario: PathBuf, seed: u64, ticks: u64) -> Result<(), AppError> {
        let process = BackendProcess::spawn(&scenario)?;
        let mut client = IpcClient::new(process);
        let mut state = AppState::new(seed, ticks);
        let mut controller = Controller::new();

        controller.init(&mut client, &mut state, seed, ticks)?;
        controller.run_to_end(&mut client, &mut state)?;
        controller.print_report(&mut client)?;

        client.shutdown();
        Ok(())
    }

    pub fn replay(scenario: PathBuf, replay_path: PathBuf) -> Result<(), AppError> {
        let process = BackendProcess::spawn(&scenario)?;
        let mut client = IpcClient::new(process);

        let replay_str = replay_path
            .to_str()
            .ok_or_else(|| AppError::Replay("invalid replay path".to_string()))?;
        client.load_replay(replay_str)?;
        let report = client.replay_to_end()?;

        use crate::screens::report_screen::ReportScreen;
        let screen = ReportScreen::new(report);
        screen.render();

        client.shutdown();
        Ok(())
    }
}
