use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::app::screen::Screen;
use crate::diagnostics::ui_error::UiError;
use crate::screens::contract_screen::ContractScreen;
use crate::screens::generate_screen::GenerateScreen;
use crate::screens::preview_screen::PreviewScreen;
use crate::screens::report_screen::ReportScreen;
use crate::transport::ipc_client::IpcClient;
use crate::transport::request::Request;
use crate::transport::response::Response;

pub struct Controller {
    state: AppState,
    client: IpcClient,
}

impl Controller {
    pub fn new(client: IpcClient) -> Self {
        Self {
            state: AppState::default(),
            client,
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), UiError> {
        match &action {
            Action::LoadContract(path) => {
                let response = self
                    .client
                    .call(Request::LoadContract { path: path.clone() })?;
                self.expect_ok(response)?;
            }
            Action::Validate => {
                let response = self.client.call(Request::ValidateContract)?;
                self.expect_ok(response)?;
            }
            Action::Preview => match self.client.call(Request::PreviewLayout)? {
                Response::Preview { files } => {
                    self.state.preview_files = files;
                }
                Response::Error {
                    message, details, ..
                } => {
                    return Err(UiError::Ipc(format!("{message}: {details}")));
                }
                _ => return Err(UiError::Ipc("unexpected preview response".to_string())),
            },
            Action::Generate { out_dir, mode } => {
                let response = self.client.call(Request::Generate {
                    out_dir: out_dir.clone(),
                    mode: mode.clone(),
                })?;
                self.expect_ok(response)?;
            }
            Action::GetManifest => match self.client.call(Request::GetManifest)? {
                Response::Manifest {
                    manifest_json,
                    manifest_hash,
                } => {
                    self.state.manifest_json = Some(manifest_json);
                    self.state.manifest_hash = Some(manifest_hash);
                }
                Response::Error {
                    message, details, ..
                } => {
                    return Err(UiError::Ipc(format!("{message}: {details}")));
                }
                _ => return Err(UiError::Ipc("unexpected manifest response".to_string())),
            },
            Action::Shutdown => {
                self.client.close();
            }
        }

        self.state = Reducer::reduce(self.state.clone(), &action);
        Ok(())
    }

    pub fn run(&mut self, contract: &str, out_dir: &str, dry_run: bool) -> Result<(), UiError> {
        self.dispatch(Action::LoadContract(contract.to_string()))?;
        self.dispatch(Action::Validate)?;
        self.dispatch(Action::Preview)?;

        self.render();

        let mode = if dry_run { "dry_run" } else { "write" };
        self.dispatch(Action::Generate {
            out_dir: out_dir.to_string(),
            mode: mode.to_string(),
        })?;
        self.dispatch(Action::GetManifest)?;
        self.render();
        self.dispatch(Action::Shutdown)?;
        Ok(())
    }

    fn render(&self) {
        match self.state.screen {
            Screen::Contract => ContractScreen::render(&self.state),
            Screen::Preview => PreviewScreen::render(&self.state),
            Screen::Generate => GenerateScreen::render(&self.state),
            Screen::Report => ReportScreen::render(&self.state),
        }
    }

    fn expect_ok(&mut self, response: Response) -> Result<(), UiError> {
        match response {
            Response::Ok => Ok(()),
            Response::Error {
                message, details, ..
            } => {
                self.state.last_error = Some(format!("{message}: {details}"));
                Err(UiError::Ipc(format!("{message}: {details}")))
            }
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }
}
