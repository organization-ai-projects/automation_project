use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::reduce;
use crate::diagnostics::error::UiError;
use crate::screens::data_screen::DataScreen;
use crate::screens::diff_screen::DiffScreen;
use crate::screens::migration_screen::MigrationScreen;
use crate::screens::report_screen::ReportScreen;
use crate::screens::schema_screen::SchemaScreen;
use crate::transport::ipc_client::IpcClient;
use crate::transport::request::Request;
use crate::transport::response::Response;
use crate::widgets::table_widget::TableWidget;
use common_json::Json;
use std::collections::HashMap;

pub struct Controller {
    pub state: AppState,
    client: IpcClient,
}

impl Controller {
    pub fn new(client: IpcClient) -> Self {
        Self {
            state: AppState::default(),
            client,
        }
    }

    pub fn load_schema(&mut self, schema: Json) -> Result<(), UiError> {
        let response = self.client.send(Request::LoadSchema {
            schema: schema.clone(),
        })?;
        match response {
            Response::Ok => {
                reduce(&mut self.state, Action::SchemaLoaded(schema));
                Ok(())
            }
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc(
                "unexpected response for LoadSchema".to_string(),
            )),
        }
    }

    pub fn insert_record(&mut self, record: Json) -> Result<(), UiError> {
        let response = self.client.send(Request::Insert { record })?;
        match response {
            Response::Ok => {
                reduce(&mut self.state, Action::RecordInserted);
                Ok(())
            }
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Insert".to_string())),
        }
    }

    pub fn validate_schema(&mut self) -> Result<(), UiError> {
        let response = self.client.send(Request::ValidateSchema)?;
        match response {
            Response::Ok => Ok(()),
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc(
                "unexpected response for ValidateSchema".to_string(),
            )),
        }
    }

    pub fn update_record(&mut self, id: u64, record: Json) -> Result<(), UiError> {
        let response = self.client.send(Request::Update { id, record })?;
        match response {
            Response::Ok => Ok(()),
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Update".to_string())),
        }
    }

    pub fn delete_record(&mut self, id: u64) -> Result<(), UiError> {
        let response = self.client.send(Request::Delete { id })?;
        match response {
            Response::Ok => Ok(()),
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Delete".to_string())),
        }
    }

    pub fn migrate_record(&mut self, id: u64, migration: Json) -> Result<(), UiError> {
        let response = self.client.send(Request::Migrate { id, migration })?;
        match response {
            Response::Ok => Ok(()),
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Migrate".to_string())),
        }
    }

    pub fn snapshot(&mut self) -> Result<(String, Json), UiError> {
        let response = self.client.send(Request::Snapshot)?;
        match response {
            Response::Snapshot { hash, snapshot } => {
                reduce(&mut self.state, Action::SnapshotReady(hash.clone()));
                Ok((hash, snapshot))
            }
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Snapshot".to_string())),
        }
    }

    pub fn diff(&mut self, from: Json, to: Json) -> Result<Json, UiError> {
        let response = self.client.send(Request::Diff { from, to })?;
        match response {
            Response::Diff { json } => Ok(json),
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Diff".to_string())),
        }
    }

    pub fn report(&mut self) -> Result<Json, UiError> {
        let response = self.client.send(Request::Report)?;
        match response {
            Response::Report { json } => {
                reduce(&mut self.state, Action::ReportReady(json.clone()));
                Ok(json)
            }
            Response::Error { message } => {
                reduce(&mut self.state, Action::Error(message.clone()));
                Err(UiError::Ipc(message))
            }
            _ => Err(UiError::Ipc("unexpected response for Report".to_string())),
        }
    }

    pub fn shutdown(&mut self) -> Result<(), UiError> {
        let response = self.client.send(Request::Shutdown)?;
        match response {
            Response::Ok => Ok(()),
            Response::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response for Shutdown".to_string())),
        }
    }
}

pub fn run_flow(schema_path: &str, record_path: &str, backend_binary: &str) -> Result<(), UiError> {
    let schema_json =
        std::fs::read_to_string(schema_path).map_err(|e| UiError::Io(e.to_string()))?;
    let record_json =
        std::fs::read_to_string(record_path).map_err(|e| UiError::Io(e.to_string()))?;

    let schema: Json =
        common_json::from_json_str(&schema_json).map_err(|e| UiError::Json(e.to_string()))?;
    let record: Json =
        common_json::from_json_str(&record_json).map_err(|e| UiError::Json(e.to_string()))?;

    let process = crate::transport::backend_process::BackendProcess::spawn(backend_binary)?;
    let client = IpcClient::new(process);
    let mut controller = Controller::new(client);
    controller.load_schema(schema.clone())?;
    controller.validate_schema()?;
    controller.insert_record(record.clone())?;
    let (before_hash, before_snapshot) = controller.snapshot()?;

    controller.update_record(1, record)?;
    controller.migrate_record(1, default_migration())?;
    let (snapshot_hash, after_snapshot) = controller.snapshot()?;
    let diff_json = controller.diff(before_snapshot, after_snapshot)?;

    controller.delete_record(1)?;
    let report_json = controller.report()?;

    let schema_screen = SchemaScreen {
        summary: format!("loaded schema from {schema_path}"),
    };
    let data_screen = DataScreen {
        summary: format!("inserted record from {record_path}"),
    };
    let migration_screen = MigrationScreen {
        summary: "migration workflow ready".to_string(),
    };
    let diff_screen = DiffScreen {
        summary: common_json::to_string(&diff_json).unwrap_or_else(|_| "{}".to_string()),
    };
    let report_screen = ReportScreen {
        summary: common_json::to_string(&report_json).unwrap_or_else(|_| "{}".to_string()),
    };
    let mut table = TableWidget::new();
    table.insert("backend", backend_binary.to_string());
    table.insert("snapshot_hash_before", before_hash);
    table.insert("snapshot_hash", snapshot_hash);

    let rendered_view = [
        schema_screen.render(),
        data_screen.render(),
        migration_screen.render(),
        diff_screen.render(),
        report_screen.render(),
        table.render(),
    ]
    .join("\n");
    controller.state.rendered_view = Some(rendered_view);

    controller.shutdown()
}

fn default_migration() -> Json {
    let mut defaults = HashMap::new();
    defaults.insert("migrated".to_string(), Json::Bool(true));

    let mut migration = HashMap::new();
    migration.insert("from_version".to_string(), Json::from(1u64));
    migration.insert("to_version".to_string(), Json::from(2u64));
    migration.insert("renames".to_string(), Json::Object(HashMap::new()));
    migration.insert("defaults".to_string(), Json::Object(defaults));
    Json::Object(migration)
}

pub fn resolve_backend_binary_path() -> Result<String, UiError> {
    if let Ok(path) = std::env::var("SCHEMA_WORLD_BACKEND_BIN")
        && !path.trim().is_empty()
    {
        return Ok(path);
    }

    let current_exe = std::env::current_exe().map_err(|e| UiError::SpawnFailed(e.to_string()))?;
    if let Some(parent) = current_exe.parent() {
        let sibling = parent.join("schema_world_backend");
        if sibling.exists() {
            return Ok(sibling.to_string_lossy().to_string());
        }
    }

    Ok("schema_world_backend".to_string())
}
