// projects/products/unstable/code_forge_engine/ui/src/app/controller.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::transport::backend_process::BackendProcess;
use crate::transport::ipc_client::IpcClient;
use anyhow::Result;

pub struct Controller {
    state: AppState,
    client: IpcClient,
}

impl Controller {
    pub fn new(process: BackendProcess) -> Self {
        Self {
            state: AppState::Idle,
            client: IpcClient::new(process),
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = Reducer::reduce(self.state.clone(), &action);
    }

    pub fn run(
        &mut self,
        contract: Option<&str>,
        out_dir: Option<&str>,
        golden_dir: Option<&str>,
    ) -> Result<()> {
        let _ = golden_dir;
        if let Some(path) = contract {
            self.dispatch(Action::LoadContract(path.to_string()));
            eprintln!("contract: {path}");
        }
        if let Some(dir) = out_dir {
            self.dispatch(Action::Generate {
                out_dir: dir.to_string(),
                mode: "dry_run".to_string(),
            });
            eprintln!("out_dir: {dir}");
        }
        Ok(())
    }
}
