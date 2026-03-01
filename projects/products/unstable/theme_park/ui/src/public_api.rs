#![allow(dead_code)]
use crate::app::app_state::AppState;
use crate::diagnostics::error::UiError;
use crate::transport::ipc_client::IpcClient;

/// Top-level controller — the only public API of the UI.
pub struct Controller {
    pub state: AppState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub fn run(
        &mut self,
        scenario: &str,
        seed: u64,
        ticks: u64,
        out_path: &str,
        replay_path: Option<&str>,
    ) -> Result<(), UiError> {
        let mut client = IpcClient::new(Some(scenario))?;

        // Load scenario.
        client.load_scenario(scenario)?;

        // Start a new run.
        client.new_run(seed, ticks)?;

        // Run to completion.
        let report_json = client.run_to_end()?;
        self.state.last_report = Some(report_json.clone());

        // Save report.
        std::fs::write(out_path, &report_json).map_err(|e| UiError::Io(e.to_string()))?;

        // Optionally save replay.
        if let Some(rpath) = replay_path {
            client.save_replay(rpath)?;
        }

        client.shutdown()?;
        Ok(())
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
