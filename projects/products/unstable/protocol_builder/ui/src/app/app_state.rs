// projects/products/unstable/protocol_builder/ui/src/app/app_state.rs
use crate::app::action::Action;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub schema_path: Option<String>,
    pub out_dir: Option<String>,
    pub manifest_hash: Option<String>,
    pub report_json: Option<String>,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::SetSchemaPath(path) => self.schema_path = Some(path),
            Action::SetOutDir(dir) => self.out_dir = Some(dir),
            Action::SetReport { manifest_hash, report_json } => {
                self.manifest_hash = Some(manifest_hash);
                self.report_json = Some(report_json);
                self.last_error = None;
            }
            Action::SetError(msg) => self.last_error = Some(msg),
            Action::ClearError => self.last_error = None,
        }
    }
}
