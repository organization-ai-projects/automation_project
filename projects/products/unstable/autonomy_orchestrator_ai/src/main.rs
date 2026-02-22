// projects/products/unstable/autonomy_orchestrator_ai/src/main.rs

mod binary_runner;
mod domain;
mod orchestrator;
mod output_writer;

use crate::domain::{BinaryInvocationSpec, Stage, TerminalState};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut output_dir = PathBuf::from("./out");
    let mut simulate_blocked = false;
    let mut timeout_ms: u64 = 30_000;
    let mut manager_bin: Option<String> = None;
    let mut manager_args: Vec<String> = Vec::new();
    let mut manager_expected_artifacts: Vec<String> = Vec::new();
    let mut executor_bin: Option<String> = None;
    let mut executor_args: Vec<String> = Vec::new();
    let mut executor_expected_artifacts: Vec<String> = Vec::new();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--simulate-blocked" => {
                simulate_blocked = true;
                i += 1;
            }
            "--timeout-ms" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                timeout_ms = args[i + 1].parse::<u64>().unwrap_or_else(|_| {
                    eprintln!("Invalid --timeout-ms value: {}", args[i + 1]);
                    process::exit(2);
                });
                i += 2;
            }
            "--manager-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_bin = Some(args[i + 1].clone());
                i += 2;
            }
            "--manager-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_args.push(args[i + 1].clone());
                i += 2;
            }
            "--manager-expected-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_expected_artifacts.push(args[i + 1].clone());
                i += 2;
            }
            "--executor-bin" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_bin = Some(args[i + 1].clone());
                i += 2;
            }
            "--executor-arg" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_args.push(args[i + 1].clone());
                i += 2;
            }
            "--executor-expected-artifact" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_expected_artifacts.push(args[i + 1].clone());
                i += 2;
            }
            val if val.starts_with("--") => {
                eprintln!("Unknown option: {val}");
                usage_and_exit();
            }
            val => {
                output_dir = PathBuf::from(val);
                i += 1;
            }
        }
    }

    let run_id = format!("run_{}", unix_timestamp_secs());
    let planning_invocation = manager_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Planning,
        command,
        args: manager_args,
        timeout_ms,
        expected_artifacts: manager_expected_artifacts,
    });
    let execution_invocation = executor_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Execution,
        command,
        args: executor_args,
        timeout_ms,
        expected_artifacts: executor_expected_artifacts,
    });

    println!("Autonomy Orchestrator AI V0");
    println!("Run ID: {}", run_id);
    println!("Output: {}", output_dir.display());
    println!("Simulate blocked: {}", simulate_blocked);
    println!("Timeout ms: {}", timeout_ms);
    println!(
        "Planning invocation configured: {}",
        planning_invocation.is_some()
    );
    println!(
        "Execution invocation configured: {}",
        execution_invocation.is_some()
    );
    println!();

    let report = Orchestrator::new(
        run_id.clone(),
        simulate_blocked,
        planning_invocation,
        execution_invocation,
    )
    .execute();

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

fn usage_and_exit() -> ! {
    eprintln!("Usage:");
    eprintln!("  autonomy_orchestrator_ai [output_dir] [options]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --simulate-blocked");
    eprintln!("  --timeout-ms <millis>");
    eprintln!("  --manager-bin <path>");
    eprintln!("  --manager-arg <value>                    (repeatable)");
    eprintln!("  --manager-expected-artifact <path>       (repeatable)");
    eprintln!("  --executor-bin <path>");
    eprintln!("  --executor-arg <value>                   (repeatable)");
    eprintln!("  --executor-expected-artifact <path>      (repeatable)");
    process::exit(2);
}
