use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::reduce;
use crate::diagnostics::error::UiError;
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
            Action::LoadMachine(machine) => self.ipc.send_load_machine(machine.clone()),
            Action::Validate => self.ipc.send_validate(),
            Action::Run(events) => self.ipc.send_run(events.clone()),
            Action::Step(event) => self.ipc.send_step(event.clone()),
            Action::TestExhaustive => {
                let report = self.ipc.send_test_exhaustive()?;
                self.state.test_report = report;
                Ok(())
            }
            Action::TestFuzz { seed, steps } => {
                let report = self.ipc.send_test_fuzz(*seed, *steps)?;
                self.state.test_report = report;
                Ok(())
            }
            Action::GetTranscript => {
                let transcript = self.ipc.send_get_transcript()?;
                self.state.transcript = transcript;
                Ok(())
            }
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
            Err(e) => {
                self.state.last_error = Some(e.to_string());
                Err(e)
            }
        }
    }
}
