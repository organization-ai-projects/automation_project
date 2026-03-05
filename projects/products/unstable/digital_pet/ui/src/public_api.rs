// projects/products/unstable/digital_pet/ui/src/public_api.rs
use crate::app::app_state::AppState;
use crate::app::controller::Controller;
use crate::diagnostics::app_error::AppError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use std::path::PathBuf;

pub struct UiApi;

impl UiApi {
    pub fn run(
        scenario: PathBuf,
        seed: u64,
        ticks: u64,
        out: Option<PathBuf>,
        replay_out: Option<PathBuf>,
    ) -> Result<(), AppError> {
        let process = BackendProcess::spawn(&scenario)?;
        let mut client = IpcClient::new(process);
        let mut state = AppState::new(seed, ticks);
        let mut controller = Controller::new();

        controller.init(&mut client, &mut state, &scenario, seed, ticks)?;
        controller.run_loop(&mut client, &mut state, replay_out.as_deref())?;

        if let Some(out_path) = out {
            controller.save_report(&mut client, &out_path)?;
        }

        client.shutdown();
        Ok(())
    }

    pub fn replay(replay: PathBuf, out: PathBuf) -> Result<(), AppError> {
        let bootstrap_scenario =
            std::env::temp_dir().join(format!("digital_pet_bootstrap_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&bootstrap_scenario);

        let process = BackendProcess::spawn(&bootstrap_scenario)?;
        let mut client = IpcClient::new(process);
        let mut controller = Controller::new();
        controller.run_replay(&mut client, &replay, &out)?;
        client.shutdown();
        Ok(())
    }
}
