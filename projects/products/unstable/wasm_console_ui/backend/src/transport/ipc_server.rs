use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;
use crate::diagnostics::error::Error;
use crate::persistence::snapshot_codec::SnapshotCodec;
use crate::plugins::builtin_plugins::BuiltinPlugins;
use crate::transport::request::Request;
use crate::transport::response::Response;
use crate::ui_model::panel_registry::PanelRegistry;

/// Backend IPC server that processes typed requests.
pub struct IpcServer {
    state: AppState,
    registry: PanelRegistry,
}

impl IpcServer {
    pub fn new() -> Self {
        let mut registry = PanelRegistry::new();
        for plugin in BuiltinPlugins::all() {
            registry.register(plugin);
        }
        Self {
            state: AppState::new(),
            registry,
        }
    }

    pub fn handle(&mut self, request: &Request) -> Response {
        match request {
            Request::LoadLogFile { path } => {
                let action = Action::LoadLogFile { path: path.clone() };
                self.state = Reducer::reduce(&self.state, &action);
                Response::OperationSuccess {
                    message: format!("Loaded log file: {path}"),
                }
            }
            Request::LoadReportFile { path } => {
                let action = Action::LoadReportFile { path: path.clone() };
                self.state = Reducer::reduce(&self.state, &action);
                Response::OperationSuccess {
                    message: format!("Loaded report file: {path}"),
                }
            }
            Request::LoadGraphFile { path } => {
                let action = Action::LoadGraphFile { path: path.clone() };
                self.state = Reducer::reduce(&self.state, &action);
                Response::OperationSuccess {
                    message: format!("Loaded graph file: {path}"),
                }
            }
            Request::DispatchAction { action_json } => {
                match common_json::from_str::<Action>(action_json) {
                    Ok(action) => {
                        self.state = Reducer::reduce(&self.state, &action);
                        Response::OperationSuccess {
                            message: "Action dispatched".to_string(),
                        }
                    }
                    Err(e) => Response::Error {
                        message: format!("Invalid action: {e}"),
                    },
                }
            }
            Request::ExportSnapshot => match SnapshotCodec::export(&self.state) {
                Ok(json) => Response::StateSnapshot { json },
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            },
            Request::ImportSnapshot { data } => match SnapshotCodec::import(data) {
                Ok(state) => {
                    self.state = state;
                    Response::OperationSuccess {
                        message: "Snapshot imported".to_string(),
                    }
                }
                Err(e) => Response::Error {
                    message: e.to_string(),
                },
            },
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn registry(&self) -> &PanelRegistry {
        &self.registry
    }
}

impl Default for IpcServer {
    fn default() -> Self {
        Self::new()
    }
}
