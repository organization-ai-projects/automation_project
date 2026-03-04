use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::{apply_response_payload, reduce};
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
        let response_payload = match &action {
            Action::LoadInputs(paths) => Some(self.ipc.send_load_inputs(paths.clone())?),
            Action::Analyze => Some(self.ipc.send_analyze()?),
            Action::RenderDocs => Some(self.ipc.send_render_docs()?),
            Action::BuildBundle => Some(self.ipc.send_build_bundle()?),
            Action::GetBundle => Some(self.ipc.send_get_bundle()?),
            Action::Quit => None,
        };

        if let Some(payload) = response_payload.as_ref() {
            apply_response_payload(&mut self.state, payload);
        }

        reduce(&mut self.state, action);
        Ok(())
    }
}
