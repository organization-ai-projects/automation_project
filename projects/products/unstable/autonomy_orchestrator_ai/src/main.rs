// projects/products/unstable/autonomy_orchestrator_ai/src/main.rs

mod domain;
mod orchestrator;
mod output_writer;

use crate::domain::TerminalState;
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut output_dir = PathBuf::from("./out");
    let mut simulate_blocked = false;

    for arg in env::args().skip(1) {
        if arg == "--simulate-blocked" {
            simulate_blocked = true;
            continue;
        }

        if arg.starts_with("--") {
            eprintln!("Unknown option: {arg}");
            eprintln!("Usage: autonomy_orchestrator_ai [output_dir] [--simulate-blocked]");
            process::exit(2);
        }

        output_dir = PathBuf::from(arg);
    }

    let run_id = format!("run_{}", unix_timestamp_secs());

    println!("Autonomy Orchestrator AI V0");
    println!("Run ID: {}", run_id);
    println!("Output: {}", output_dir.display());
    println!("Simulate blocked: {}", simulate_blocked);
    println!();

    let report = Orchestrator::new(run_id.clone(), simulate_blocked).execute();

    if let Err(err) = write_run_report(&report, &output_dir) {
        eprintln!("Failed to write run report: {err}");
        process::exit(1);
    }

    let report_path = output_dir.join("orchestrator_run_report.json");
    println!("Run report: {}", report_path.display());
    println!("Terminal state: {:?}", report.terminal_state);

    match report.terminal_state {
        Some(TerminalState::Done) => process::exit(0),
        Some(TerminalState::Blocked) => process::exit(3),
        Some(TerminalState::Timeout) => process::exit(124),
        Some(TerminalState::Failed) | None => process::exit(1),
    }
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
