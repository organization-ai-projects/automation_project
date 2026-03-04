// projects/products/unstable/protocol_builder/ui/src/app/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::reduce;
use crate::diagnostics::ui_error::UiError;
use crate::transport::ipc_client::IpcClient;

pub struct Controller {
    pub state: AppState,
    client: IpcClient,
}

impl Controller {
    pub fn new(client: IpcClient) -> Self {
        Self {
            state: AppState::new(),
            client,
        }
    }

    pub fn generate(&mut self, schema_path: &str, out_dir: &str) -> Result<(), UiError> {
        reduce(
            &mut self.state,
            Action::SetSchemaPath(schema_path.to_string()),
        );
        reduce(&mut self.state, Action::SetOutDir(out_dir.to_string()));
        reduce(&mut self.state, Action::ClearError);

        self.client.send_load_schema(schema_path).inspect_err(|e| {
            reduce(&mut self.state, Action::SetError(e.to_string()));
        })?;
        self.client.send_validate().inspect_err(|e| {
            reduce(&mut self.state, Action::SetError(e.to_string()));
        })?;

        // Dry-run first to ensure deterministic in-memory generation path stays healthy.
        let _ = self.client.send_generate_dry_run().inspect_err(|e| {
            reduce(&mut self.state, Action::SetError(e.to_string()));
        })?;
        let (manifest_hash, report_json) =
            self.client.send_generate_write(out_dir).inspect_err(|e| {
                reduce(&mut self.state, Action::SetError(e.to_string()));
            })?;
        reduce(
            &mut self.state,
            Action::SetReport {
                manifest_hash,
                report_json,
            },
        );
        self.client.send_shutdown().inspect_err(|e| {
            reduce(&mut self.state, Action::SetError(e.to_string()));
        })?;
        Ok(())
    }
}
