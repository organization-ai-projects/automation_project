use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::diagnostics::error::UiError;
use crate::transport::ipc_client::IpcClient;

pub struct Controller {
    pub state: AppState,
    ipc: IpcClient,
}

impl Controller {
    pub fn new(ipc: IpcClient) -> Self {
        Self {
            state: AppState::default(),
            ipc,
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), UiError> {
        match action {
            Action::LoadScenario(scenario) => {
                self.ipc.send_load_scenario(scenario)?;
                Reducer::set_scenario_loaded(&mut self.state);
            }
            Action::NewRun { seed } => {
                self.ipc.send_new_run(seed)?;
                Reducer::set_run_active(&mut self.state);
            }
            Action::EncounterStep | Action::StartEncounter => {
                let json = self.ipc.send_encounter_step()?;
                Reducer::set_encounter(&mut self.state, json);
            }
            Action::CaptureAttempt => {
                let json = self.ipc.send_capture_attempt()?;
                Reducer::set_encounter(&mut self.state, json);
            }
            Action::StartBattle => {
                let json = self.ipc.send_start_battle()?;
                Reducer::set_battle(&mut self.state, json);
            }
            Action::BattleAction(action_str) => {
                let json = self.ipc.send_battle_action(action_str)?;
                Reducer::set_battle(&mut self.state, json);
            }
            Action::BattleStep => {
                let json = self.ipc.send_battle_step()?;
                Reducer::set_battle(&mut self.state, json);
            }
            Action::EndBattle => {
                self.ipc.send_end_battle()?;
                Reducer::set_battle_ended(&mut self.state);
            }
            Action::GetSnapshot => {
                let (hash, json) = self.ipc.send_get_snapshot()?;
                Reducer::set_snapshot(&mut self.state, hash, json);
            }
            Action::GetReport => {
                let (hash, json) = self.ipc.send_get_report()?;
                Reducer::set_report(&mut self.state, hash, json);
            }
            Action::SaveReplay => {
                let replay = self.ipc.send_save_replay()?;
                Reducer::set_replay(&mut self.state, replay);
            }
            Action::LoadReplay(replay) => {
                self.ipc.send_load_replay(replay)?;
            }
            Action::ReplayToEnd => {
                self.ipc.send_replay_to_end()?;
            }
            Action::Quit => {
                self.ipc.close();
            }
        }
        Ok(())
    }
}
