// projects/products/unstable/protocol_builder/ui/src/app/controller.rs
use anyhow::Result;

use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::reduce;
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

    pub fn generate(&mut self, schema_path: &str, out_dir: &str) -> Result<()> {
        reduce(&mut self.state, Action::SetSchemaPath(schema_path.to_string()));
        reduce(&mut self.state, Action::SetOutDir(out_dir.to_string()));

        self.client.send_load_schema(schema_path)?;
        self.client.send_validate()?;
        let (manifest_hash, report_json) = self.client.send_generate_write(out_dir)?;
        reduce(
            &mut self.state,
            Action::SetReport { manifest_hash, report_json },
        );
        self.client.send_shutdown()?;
        Ok(())
    }
}
