#![allow(dead_code)]
use crate::app::app_state::AppState;
use crate::diagnostics::error::UiError;
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

    pub fn state(&self) -> &AppState {
        &self.state
    }
}
