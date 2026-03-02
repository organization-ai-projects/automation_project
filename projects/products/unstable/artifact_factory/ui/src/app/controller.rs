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
        match &action {
            Action::LoadInputs(paths) => {
                self.ipc.send_load_inputs(paths.clone())?;
            }
            Action::Analyze => {
                self.ipc.send_analyze()?;
            }
            Action::RenderDocs => {
                self.ipc.send_render_docs()?;
            }
            Action::BuildBundle => {
                self.ipc.send_build_bundle()?;
            }
            Action::GetBundle => {
                let resp = self.ipc.send_get_bundle()?;
                if let Some((hash, manifest)) = resp {
                    self.state.bundle_hash = Some(hash);
                    self.state.bundle_manifest = manifest;
                }
            }
            Action::Quit => {}
        }
        reduce(&mut self.state, action);
        Ok(())
    }
}
