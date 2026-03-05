use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::controller::Controller as AppController;
use crate::diagnostics::ui_error::UiError;
use crate::fixtures::fixture_loader::FixtureLoader;
use crate::transport::ipc_client::IpcClient;

pub struct Controller {
    flow: AppController,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            flow: AppController::new(),
        }
    }

    pub fn run_pack(
        &mut self,
        pack_kind: &str,
        seed: u64,
        ticks: u64,
        out_path: &str,
    ) -> Result<(), UiError> {
        self.flow
            .dispatch(Action::SelectPack(pack_kind.to_string()));
        self.flow.dispatch(Action::SetSeed(seed));
        self.flow.dispatch(Action::SetTicks(ticks));
        self.flow.dispatch(Action::StartRun);
        let mut client = IpcClient::new()?;
        let report = client.new_run(pack_kind, seed, ticks)?;
        std::fs::write(out_path, &report).map_err(|e| UiError::Io(e.to_string()))?;
        let _ = FixtureLoader::load_report(out_path)?;
        let _ = client.shutdown();
        self.flow.dispatch(Action::Shutdown);
        Ok(())
    }

    pub fn replay_to_report(&mut self, replay_path: &str, out_path: &str) -> Result<(), UiError> {
        self.flow.dispatch(Action::StartRun);
        let mut client = IpcClient::new()?;
        let _ = client.load_replay(replay_path)?;
        let replay_report = client.replay_to_end()?;
        std::fs::write(out_path, &replay_report).map_err(|e| UiError::Io(e.to_string()))?;
        let _ = FixtureLoader::load_report(out_path)?;
        let _ = client.shutdown();
        self.flow.dispatch(Action::Shutdown);
        Ok(())
    }

    pub fn inspect_replay(&mut self, replay_path: &str, query: &str) -> Result<String, UiError> {
        self.flow.dispatch(Action::StartRun);
        let mut client = IpcClient::new()?;
        let _ = client.load_replay(replay_path)?;
        let response = client.query(query)?;
        if !response.starts_with('{') {
            return Err(UiError::Serialization(
                "backend returned non-JSON payload".to_string(),
            ));
        }
        let _ = client.shutdown();
        self.flow.dispatch(Action::Shutdown);
        Ok(response)
    }

    pub fn run_pack_with_replay(
        &mut self,
        pack_kind: &str,
        seed: u64,
        ticks: u64,
        out_path: &str,
        replay_out: Option<&str>,
    ) -> Result<(), UiError> {
        self.flow
            .dispatch(Action::SelectPack(pack_kind.to_string()));
        self.flow.dispatch(Action::SetSeed(seed));
        self.flow.dispatch(Action::SetTicks(ticks));
        self.flow.dispatch(Action::StartRun);
        let mut client = IpcClient::new()?;
        let report = client.new_run(pack_kind, seed, ticks)?;
        if let Some(replay_path) = replay_out {
            let _ = client.save_replay(replay_path)?;
        }
        std::fs::write(out_path, &report).map_err(|e| UiError::Io(e.to_string()))?;
        let _ = FixtureLoader::load_report(out_path)?;
        let _ = client.shutdown();
        self.flow.dispatch(Action::Shutdown);
        Ok(())
    }

    pub fn state(&self) -> &AppState {
        self.flow.state()
    }
}
