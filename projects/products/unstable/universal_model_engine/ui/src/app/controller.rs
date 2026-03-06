use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::reduce;
use crate::diagnostics::ui_error::UiError;
use crate::transport::ipc_client::IpcClient;

pub struct Controller {
    pub state: AppState,
    pub ipc: IpcClient,
}

impl Controller {
    pub fn new(ipc: IpcClient) -> Self {
        Self {
            state: AppState::new(),
            ipc,
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), UiError> {
        let result = match &action {
            Action::LoadModel(model) => self.ipc.send_load_model(model.clone()),
            Action::ValidateModel => self.ipc.send_validate_model(),
            Action::NewRun { seed } => self.ipc.send_new_run(*seed),
            Action::Step => self.ipc.send_step(),
            Action::RunToEnd => self.ipc.send_run_to_end(),
            Action::GetSnapshot => {
                let snapshot = self.ipc.send_get_snapshot()?;
                if let Some((snapshot_hash, snapshot_json)) = snapshot {
                    self.state.snapshot_hash = Some(snapshot_hash);
                    self.state.snapshot_json = Some(snapshot_json);
                }
                Ok(())
            }
            Action::GetReport => {
                let report = self.ipc.send_get_report()?;
                if let Some((run_hash, report_json)) = report {
                    self.state.run_hash = Some(run_hash);
                    self.state.last_report = Some(report_json);
                }
                Ok(())
            }
            Action::SaveReplay => {
                self.ipc.send_save_replay()?;
                self.state.replay_saved = true;
                self.state.replay_data = self.ipc.send_get_replay()?;
                Ok(())
            }
            Action::LoadReplay(replay) => self.ipc.send_load_replay(replay.clone()),
            Action::ReplayToEnd => self.ipc.send_replay_to_end(),
            Action::Quit => {
                self.ipc.close();
                Ok(())
            }
        };

        match result {
            Ok(()) => {
                self.state.last_error = None;
                reduce(&mut self.state, action);
                Ok(())
            }
            Err(error) => {
                self.state.last_error = Some(error.to_string());
                Err(error)
            }
        }
    }
}
