//! projects/products/unstable/market_tycoon/ui/src/screens/replay_screen.rs
use std::{env, error, fs, process};

use crate::components::{SimReportView, StatusBanner};
pub(crate) struct ReplayScreen;

impl ReplayScreen {
    pub(crate) fn execute(args: &[String]) -> Result<(), Box<dyn error::Error>> {
        let mut replay_path = None;
        let mut out_path = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--replay" => {
                    i += 1;
                    replay_path = args.get(i).map(String::clone);
                }
                "--out" => {
                    i += 1;
                    out_path = args.get(i).map(String::clone);
                }
                _ => {}
            }
            i += 1;
        }

        let replay = replay_path.ok_or("--replay is required")?;
        let out = out_path.ok_or("--out is required")?;

        StatusBanner::print("Starting replay...");

        let backend_bin = env::var("MARKET_TYCOON_BACKEND_BIN")
            .unwrap_or_else(|_| "market_tycoon_backend".to_string());

        let status = process::Command::new(&backend_bin)
            .args(["replay", "--replay", &replay, "--out", &out])
            .status()
            .map_err(|e| format!("Failed to run backend replay: {e}"))?;

        if status.success() {
            let report_data =
                fs::read_to_string(&out).map_err(|e| format!("Failed to read report: {e}"))?;
            SimReportView::display(&report_data);
            StatusBanner::print("Replay completed successfully.");
        } else {
            StatusBanner::print(&format!("Backend exited with code: {:?}", status.code()));
        }

        Ok(())
    }
}
