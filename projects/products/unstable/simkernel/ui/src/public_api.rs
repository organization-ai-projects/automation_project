use crate::app::app_state::AppState;
use crate::diagnostics::ui_error::UiError;
use crate::transport::ipc_client::IpcClient;

pub struct Controller {
    state: AppState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub fn run_pack(
        &mut self,
        pack_kind: &str,
        seed: u64,
        ticks: u64,
        out_path: &str,
    ) -> Result<(), UiError> {
        let mut client = IpcClient::new()?;
        let report = client.new_run(pack_kind, seed, ticks)?;
        self.state.last_report = Some(report.clone());
        std::fs::write(out_path, &report).map_err(|e| UiError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn replay_to_report(&mut self, replay_path: &str, out_path: &str) -> Result<(), UiError> {
        let mut client = IpcClient::new()?;
        let _ = client.load_replay(replay_path)?;
        let replay_report = client.replay_to_end()?;
        self.state.last_report = Some(replay_report.clone());
        std::fs::write(out_path, &replay_report).map_err(|e| UiError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn inspect_replay(&mut self, replay_path: &str, query: &str) -> Result<String, UiError> {
        let mut client = IpcClient::new()?;
        let _ = client.load_replay(replay_path)?;
        client.query(query)
    }

    pub fn run_pack_with_replay(
        &mut self,
        pack_kind: &str,
        seed: u64,
        ticks: u64,
        out_path: &str,
        replay_out: Option<&str>,
    ) -> Result<(), UiError> {
        let mut client = IpcClient::new()?;
        let report = client.new_run(pack_kind, seed, ticks)?;
        if let Some(replay_path) = replay_out {
            let _ = client.save_replay(replay_path)?;
        }
        self.state.last_report = Some(report.clone());
        std::fs::write(out_path, &report).map_err(|e| UiError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}
