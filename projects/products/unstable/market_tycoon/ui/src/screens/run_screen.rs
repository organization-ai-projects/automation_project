//! projects/products/unstable/market_tycoon/ui/src/screens/run_screen.rs
use std::{env, error, fs, process};

use crate::components::{RunControls, SimReportView, StatusBanner};

pub(crate) struct RunScreen;

impl RunScreen {
    pub(crate) fn execute(args: &[String]) -> Result<(), Box<dyn error::Error>> {
        let mut scenario_path = None;
        let mut seed: u64 = 42;
        let mut ticks: u64 = 100;
        let mut out_path = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--scenario" => {
                    i += 1;
                    scenario_path = args.get(i).cloned();
                }
                "--seed" => {
                    i += 1;
                    seed = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(42);
                }
                "--ticks" => {
                    i += 1;
                    ticks = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(100);
                }
                "--out" => {
                    i += 1;
                    out_path = args.get(i).cloned();
                }
                _ => {}
            }
            i += 1;
        }

        let scenario = scenario_path.ok_or("--scenario is required")?;
        let out = out_path.ok_or("--out is required")?;

        // Use StatusBanner to display information
        StatusBanner::print("Starting simulation run...");
        let controls = RunControls::new(seed, ticks);
        StatusBanner::print(&format!(
            "Seed: {}, Ticks: {}",
            controls.seed(),
            controls.ticks()
        ));

        let backend_bin = env::var("MARKET_TYCOON_BACKEND_BIN")
            .unwrap_or_else(|_| "market_tycoon_backend".to_string());

        let status = process::Command::new(&backend_bin)
            .args([
                "run",
                "--ticks",
                &ticks.to_string(),
                "--seed",
                &seed.to_string(),
                "--scenario",
                &scenario,
                "--out",
                &out,
            ])
            .status()
            .map_err(|e| format!("Failed to run backend: {e}"))?;

        if status.success() {
            let report_data =
                fs::read_to_string(&out).map_err(|e| format!("Failed to read report: {e}"))?;
            SimReportView::display(&report_data);
            StatusBanner::print("Run completed successfully.");
        } else {
            StatusBanner::print(&format!("Backend exited with code: {:?}", status.code()));
        }

        Ok(())
    }
}
