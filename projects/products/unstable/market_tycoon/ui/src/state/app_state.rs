//! projects/products/unstable/market_tycoon/ui/src/state/app_state.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct AppState {
    status: String,
    report: Option<String>,
    scenario_path: Option<String>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn status(&self) -> &str {
        &self.status
    }

    pub(crate) fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub(crate) fn report(&self) -> Option<&str> {
        self.report.as_deref()
    }

    pub(crate) fn set_report(&mut self, report: String) {
        self.report = Some(report);
    }

    pub(crate) fn scenario_path(&self) -> Option<&str> {
        self.scenario_path.as_deref()
    }

    pub(crate) fn set_scenario_path(&mut self, path: String) {
        self.scenario_path = Some(path);
    }

    pub(crate) fn run_simulation(&mut self) {
        self.set_status("Simulation started".to_string());
        println!("Status: {}", self.status());
        // Simulate setting a report
        self.set_report("Simulation report data".to_string());
        println!("Report: {:?}", self.report());
    }
}
