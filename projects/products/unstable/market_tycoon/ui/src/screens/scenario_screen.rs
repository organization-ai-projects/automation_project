//! projects/products/unstable/market_tycoon/ui/src/screens/scenario_screen.rs
use std::{env, error, process};

use crate::components::StatusBanner;

pub(crate) struct ScenarioScreen;

impl ScenarioScreen {
    pub(crate) fn execute(args: &[String]) -> Result<(), Box<dyn error::Error>> {
        let mut scenario_path = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scenario" => {
                    i += 1;
                    scenario_path = args.get(i).map(String::clone);
                }
                _ => {}
            }
            i += 1;
        }

        let scenario = scenario_path.ok_or("--scenario is required")?;

        StatusBanner::print("Validating scenario...");

        let backend_bin = env::var("MARKET_TYCOON_BACKEND_BIN")
            .unwrap_or_else(|_| "market_tycoon_backend".to_string());

        let status = process::Command::new(&backend_bin)
            .args(["validate", "--scenario", &scenario])
            .status()
            .map_err(|e| format!("Failed to validate scenario: {e}"))?;

        if status.success() {
            StatusBanner::print("Scenario is valid.");
        } else {
            StatusBanner::print(&format!(
                "Scenario validation failed (exit code: {:?})",
                status.code()
            ));
        }

        Ok(())
    }
}
