use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    status: String,
    report: Option<String>,
    scenario_path: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn report(&self) -> Option<&str> {
        self.report.as_deref()
    }

    pub fn set_report(&mut self, report: String) {
        self.report = Some(report);
    }

    pub fn scenario_path(&self) -> Option<&str> {
        self.scenario_path.as_deref()
    }

    pub fn set_scenario_path(&mut self, path: String) {
        self.scenario_path = Some(path);
    }
}
