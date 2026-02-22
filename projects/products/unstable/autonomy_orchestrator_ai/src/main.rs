// projects/products/unstable/autonomy_orchestrator_ai/src/main.rs

mod binary_runner;
mod checkpoint_store;
mod domain;
mod orchestrator;
mod output_writer;

use crate::checkpoint_store::load_checkpoint;
use crate::domain::{BinaryInvocationSpec, OrchestratorCheckpoint, Stage, TerminalState};
use crate::orchestrator::Orchestrator;
use crate::output_writer::write_run_report;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut output_dir = PathBuf::from("./out");
    let mut simulate_blocked = false;
    let mut resume = false;
    let mut timeout_ms: u64 = 30_000;
    let mut checkpoint_path_override: Option<PathBuf> = None;
    let mut manager_bin: Option<String> = None;
    let mut manager_args: Vec<String> = Vec::new();
    let mut manager_env: Vec<(String, String)> = Vec::new();
    let mut manager_expected_artifacts: Vec<String> = Vec::new();
    let mut executor_bin: Option<String> = None;
    let mut executor_args: Vec<String> = Vec::new();
    let mut executor_env: Vec<(String, String)> = Vec::new();
    let mut executor_expected_artifacts: Vec<String> = Vec::new();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--simulate-blocked" => {
                simulate_blocked = true;
                i += 1;
            }
            "--resume" => {
                resume = true;
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
            "--checkpoint-path" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                checkpoint_path_override = Some(PathBuf::from(args[i + 1].clone()));
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
            "--manager-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_env.push(parse_env_pair(&args[i + 1]));
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
            "--executor-env" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_env.push(parse_env_pair(&args[i + 1]));
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

    let checkpoint_path = checkpoint_path_override
        .clone()
        .unwrap_or_else(|| output_dir.join("orchestrator_checkpoint.json"));
    let checkpoint = if resume {
        match load_checkpoint(&checkpoint_path) {
            Ok(cp) => Some(cp),
            Err(err) => {
                eprintln!("Failed to resume checkpoint: {err}");
                process::exit(1);
            }
        }
    } else {
        None
    };
    let run_id = checkpoint
        .as_ref()
        .map(|cp: &OrchestratorCheckpoint| cp.run_id.clone())
        .unwrap_or_else(|| format!("run_{}", unix_timestamp_secs()));
    let planning_invocation = manager_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Planning,
        command,
        args: manager_args,
        env: manager_env,
        timeout_ms,
        expected_artifacts: manager_expected_artifacts,
    });
    let execution_invocation = executor_bin.map(|command| BinaryInvocationSpec {
        stage: Stage::Execution,
        command,
        args: executor_args,
        env: executor_env,
        timeout_ms,
        expected_artifacts: executor_expected_artifacts,
    });

    println!("Autonomy Orchestrator AI V0");
    println!("Run ID: {}", run_id);
    println!("Output: {}", output_dir.display());
    println!("Resume: {}", resume);
    println!("Checkpoint path: {}", checkpoint_path.display());
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
        checkpoint,
        Some(checkpoint_path),
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
    eprintln!("  --resume");
    eprintln!("  --checkpoint-path <path>");
    eprintln!("  --simulate-blocked");
    eprintln!("  --timeout-ms <millis>");
    eprintln!("  --manager-bin <path>");
    eprintln!("  --manager-arg <value>                    (repeatable)");
    eprintln!("  --manager-env <KEY=VALUE>               (repeatable)");
    eprintln!("  --manager-expected-artifact <path>       (repeatable)");
    eprintln!("  --executor-bin <path>");
    eprintln!("  --executor-arg <value>                   (repeatable)");
    eprintln!("  --executor-env <KEY=VALUE>              (repeatable)");
    eprintln!("  --executor-expected-artifact <path>      (repeatable)");
    process::exit(2);
}

fn parse_env_pair(raw: &str) -> (String, String) {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        eprintln!("Invalid env pair '{}', expected KEY=VALUE", raw);
        process::exit(2);
    }
    (key.to_string(), value.unwrap_or_default().to_string())
}
